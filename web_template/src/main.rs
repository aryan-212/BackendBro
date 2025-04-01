
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Song {
    id: u64,
    title: String,
    artist: String,
    rank: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    songs: HashMap<u64, Song>,
}

impl Database {
    fn new() -> Self {
        Self {
            songs: HashMap::new(),
        }
    }

    // CRUD DATA for Songs
    fn insert_song(&mut self, song: Song) {
        self.songs.insert(song.id, song);
    }

    fn get_song(&self, id: &u64) -> Option<&Song> {
        self.songs.get(id)
    }

    fn get_all_songs(&self) -> Vec<&Song> {
        self.songs.values().collect()
    }

    fn delete_song(&mut self, id: &u64) {
        self.songs.remove(id);
    }

    fn update_song(&mut self, song: Song) {
        self.songs.insert(song.id, song);
    }

    // Load / Save
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("songs_database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("songs_database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

// Handlers for Songs
async fn create_song(app_state: web::Data<AppState>, song: web::Json<Song>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_song(song.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn get_song(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_song(&id.into_inner()) {
        Some(song) => HttpResponse::Ok().json(song),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn get_all_songs(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let songs = db.get_all_songs();
    HttpResponse::Ok().json(songs)
}

async fn update_song(app_state: web::Data<AppState>, song: web::Json<Song>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.update_song(song.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_song(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.delete_song(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState { db: Mutex::new(db) });

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
            .app_data(data.clone())
            .route("/song", web::post().to(create_song))
            .route("/song", web::get().to(get_all_songs))
            .route("/song", web::put().to(update_song))
            .route("/song/{id}", web::get().to(get_song))
            .route("/song/{id}", web::delete().to(delete_song))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
