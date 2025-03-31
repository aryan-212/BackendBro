
use actix_cors::Cors;
use actix_web::{
    middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::sync::Mutex;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VideoMetadata {
    id: u64,
    title: String,
    description: String,
    // Could also include duration, resolution, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Database {
    videos: std::collections::HashMap<u64, VideoMetadata>,
}

impl Database {
    fn new() -> Self {
        Self {
            videos: std::collections::HashMap::new(),
        }
    }

    fn insert_video(&mut self, video: VideoMetadata) {
        self.videos.insert(video.id, video);
    }

    fn get_video(&self, id: &u64) -> Option<&VideoMetadata> {
        self.videos.get(id)
    }

    fn get_all_videos(&self) -> Vec<&VideoMetadata> {
        self.videos.values().collect::<Vec<_>>()
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

async fn upload_video(
    req: HttpRequest,
    payload: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // multipart/form-data is handled slightly differently
    let mut body = web::BytesMut::new();
    let mut stream = payload;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // Save the video to disk (e.g., using a unique filename)
    let filename = format!("{}.mp4", chrono::Utc::now().timestamp()); // Example filename
    let filepath = format!("uploads/{}", filename);

    std::fs::create_dir_all("uploads").unwrap(); // Create the uploads directory if it doesn't exist

    let mut file = fs::File::create(filepath).expect("Unable to create file");
    file.write_all(&body).expect("Unable to write data");

    // Optionally, store metadata in the database
    let mut db = app_state.db.lock().unwrap();
    let video_metadata = VideoMetadata {
        id: chrono::Utc::now().timestamp() as u64,
        title: filename.clone(), // Or get from request
        description: String::from(""), // Or get from request
    };
    db.insert_video(video_metadata);
    let _ = db.save_to_file();

    Ok(HttpResponse::Ok().body(format!("Video uploaded successfully: {}", filename)))
}

async fn get_video(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_video(&id.into_inner()) {
        Some(video) => {
             // Serve the video file using actix-files or similar.  Need a static files setup
            HttpResponse::Ok().json(video)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

async fn list_videos(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let videos = db.get_all_videos();
    HttpResponse::Ok().json(videos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Enable logger
            .wrap(
                Cors::permissive() // Adjust CORS settings as needed
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .service(web::resource("/upload").route(web::post().to(upload_video)))
            .service(web::resource("/videos/{id}").route(web::get().to(get_video)))
            .service(web::resource("/videos").route(web::get().to(list_videos)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
