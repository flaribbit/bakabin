use std::sync::{Arc, Mutex};

use axum::{
    extract::{Multipart, State, TypedHeader},
    headers::Cookie,
    routing::{get, post},
    Router,
};

#[derive(Clone)]
struct AppState {
    token: String,
    data: Arc<Mutex<String>>,
}

async fn upload(mut multipart: Multipart) {
    use std::io::Write;
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filepath = format!("uploads/{}", uuid::Uuid::new_v4().to_string());
        let mut file = std::fs::File::create(filepath).unwrap();
        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk).unwrap();
        }
    }
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
    let app_state = AppState {
        token: uuid::Uuid::new_v4().to_string(),
        data: Arc::new(Mutex::new("".to_owned())),
    };
    println!("token: {}", app_state.token);
    let app = Router::new()
        .route("/upload", post(upload))
        .route("/test", get(test))
        .with_state(app_state);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
