use axum::{
    extract::{DefaultBodyLimit, Multipart, State, TypedHeader},
    headers::Cookie,
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

static IMAGE_PATH: &str = "image";

#[derive(Clone)]
struct AppState {
    token: String,
    data: Arc<Mutex<String>>,
}

async fn upload(mut multipart: Multipart) -> String {
    use std::io::Write;
    use std::path::Path;
    let mut ret = Vec::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let original_name = field.file_name().unwrap();
        dbg!(original_name);
        let ext = Path::new(original_name)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap();
        let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);
        println!("{} -> {}", original_name, filename);
        let mut file = std::fs::File::create(format!("{}/{}", IMAGE_PATH, filename)).unwrap();
        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk).unwrap();
        }
        ret.push(filename);
    }
    ret.join("\n")
}

async fn test(State(state): State<AppState>, TypedHeader(cookie): TypedHeader<Cookie>) -> String {
    let token = cookie.get("token").unwrap_or_default();
    if token == state.token {
        "pass".to_owned()
    } else {
        "fail".to_owned()
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    if !std::path::Path::new(IMAGE_PATH).exists() {
        println!("Creating images directory");
        std::fs::create_dir(IMAGE_PATH).unwrap();
    }
    let app_state = AppState {
        token: uuid::Uuid::new_v4().to_string(),
        data: Arc::new(Mutex::new("".to_owned())),
    };
    println!(
        "TIP: Run this code in console to login:\ndocument.cookie='token={}'",
        app_state.token
    );
    let app = Router::new()
        .route("/upload", post(upload))
        .route("/test", get(test))
        .layer(DefaultBodyLimit::disable())
        .nest_service("/image", ServeDir::new(IMAGE_PATH))
        .with_state(app_state);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
