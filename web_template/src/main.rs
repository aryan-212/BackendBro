use actix_cors::Cors;

use actix_web::{App, HttpResponse, HttpServer, Responder, http::header, web};

use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Video {
    id: u64,
    title: String,
    description: String,
    url: String, // URL to the video file
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    videos: HashMap<u64, Video>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            videos: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // VIDEO CRUD OPERATIONS
    fn insert_video(&mut self, video: Video) {
        self.videos.insert(video.id, video);
    }

    fn get_video(&self, id: &u64) -> Option<&Video> {
        self.videos.get(id)
    }

    fn get_all_videos(&self) -> Vec<&Video> {
        self.videos.values().collect()
    }

    fn delete_video(&mut self, id: &u64) {
        self.videos.remove(id);
    }

    fn update_video(&mut self, video: Video) {
        self.videos.insert(video.id, video);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING AND LOADING
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
    video_dir: String,
}

// Helper function to ensure video directory exists
fn ensure_video_directory_exists(video_dir: &str) -> std::io::Result<()> {
    let path: &Path = Path::new(video_dir);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

// VIDEO HANDLERS
async fn create_video(app_state: web::Data<AppState>, video: web::Json<Video>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_video(video.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn get_video(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_video(&id.into_inner()) {
        Some(video) => HttpResponse::Ok().json(video),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn list_videos(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let videos: Vec<&Video> = db.get_all_videos();
    HttpResponse::Ok().json(videos)
}

async fn update_video(app_state: web::Data<AppState>, video: web::Json<Video>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update_video(video.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_video(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete_video(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

// AUTHENTICATION HANDLERS
async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!")
        }
        _ => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set the video directory
    let video_dir: String = String::from("videos"); // Store videos in ./videos directory
    ensure_video_directory_exists(&video_dir)?; // Create the directory if it doesn't exist

    // Load database
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let app_state: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db),
        video_dir: video_dir.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(app_state.clone())
            // Video routes
            .route("/videos", web::get().to(list_videos))
            .route("/videos/{id}", web::get().to(get_video))
            .route("/videos", web::post().to(create_video))
            .route("/videos", web::put().to(update_video))
            .route("/videos/{id}", web::delete().to(delete_video))
            // Authentication routes
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
