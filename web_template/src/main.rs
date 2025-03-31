use actix_cors::Cors;
use actix_web::{
    middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Data structure to hold video metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Video {
    id: u64,
    title: String,
    description: String,
    url: String, // URL to the video file
}

// Simple in-memory database (replace with a real database for production)
struct AppState {
    videos: Mutex<Vec<Video>>,
}

// Custom Responder for streaming video
struct VideoStream {
    path: String,
}

impl Responder for VideoStream {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let file = actix_files::NamedFile::open_async(self.path);

        match file {
            Ok(file) => ok(file.into_response(_req)),
            Err(_) => ok(HttpResponse::NotFound().finish()),
        }
    }
}

// Handler for adding a new video
async fn add_video(
    app_state: web::Data<AppState>,
    video: web::Json<Video>,
) -> impl Responder {
    let mut videos = app_state.videos.lock().unwrap();
    videos.push(video.into_inner());
    HttpResponse::Ok().body("Video added")
}

// Handler for listing all videos
async fn list_videos(app_state: web::Data<AppState>) -> impl Responder {
    let videos = app_state.videos.lock().unwrap();
    HttpResponse::Ok().json(videos.clone())
}

// Handler for streaming a video
async fn stream_video(
    app_state: web::Data<AppState>,
    video_id: web::Path<u64>,
) -> impl Responder {
    let videos = app_state.videos.lock().unwrap();
    let video = videos.iter().find(|v| v.id == *video_id);

    match video {
        Some(video) => {
            // Validate URL to prevent path traversal vulnerabilities
            if !video.url.starts_with("videos/") {
                return HttpResponse::BadRequest().body("Invalid video URL");
            }
            VideoStream { path: video.url.clone() }
        }
        None => HttpResponse::NotFound().body("Video not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the app state with some dummy data
    let app_state = web::Data::new(AppState {
        videos: Mutex::new(vec![
            Video {
                id: 1,
                title: "Sample Video".to_string(),
                description: "A sample video for testing".to_string(),
                url: "videos/sample.mp4".to_string(), //  Store video files in a 'videos' directory
            },
            Video {
                id: 2,
                title: "Another Video".to_string(),
                description: "Another video for demonstration".to_string(),
                url: "videos/another.mp4".to_string(),
            },
        ]),
    });

    // Ensure the "videos" directory exists
    std::fs::create_dir_all("videos").expect("Failed to create videos directory");

    println!("Starting server on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Enable request logging
            .wrap(
                Cors::permissive() //Allow all origins
                //.allowed_origin_fn(|origin, _req_head| {
                //  origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                //})
                // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                // .allowed_header(header::CONTENT_TYPE)
                // .supports_credentials()
                // .max_age(3600),
            )
            .app_data(app_state.clone())
            .route("/videos", web::get().to(list_videos))
            .route("/videos/{video_id}", web::get().to(stream_video))
            .route("/videos", web::post().to(add_video)) // Route to add new video
            // Serve static files from the "public" directory (create this directory)
            .service(actix_files::Files::new("/", "./public").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
