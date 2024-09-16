use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust Backend!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn register_user(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(user.into_inner());
    HttpResponse::Ok().body("User registered successfully")
}

async fn login_user(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let users = data.users.lock().unwrap();
    for u in users.iter() {
        if u.username == user.username && u.password == user.password {
            return HttpResponse::Ok().body("Login successful");
        }
    }
    HttpResponse::Unauthorized().body("Invalid username or password")
}

async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let users_without_passwords: Vec<String> = users.iter()
        .map(|user| user.username.clone())
        .collect();
    HttpResponse::Ok().json(users_without_passwords)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    println!("Server running at http://localhost:8000");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .route("/", web::get().to(hello))
            .route("/echo", web::post().to(echo))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/users", web::get().to(get_users))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}