use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

fn read_config() -> Config {
    use std::io::stdin;
    use std::io::Write;
    use std::path::Path;
    let config_path = Path::new("config.toml");
    // check if `config.toml` exists
    // ask for username and password
    // then store it in `config.toml`
    if !Path::new("config.toml").exists() {
        let mut config = Config { users: Vec::new() };
        let mut input = String::new();
        println!("No config file found. Creating one...");
        println!("Enter username:");
        stdin().read_line(&mut input).unwrap();
        let username = input.trim().to_string();
        input.clear();
        println!("Enter password:");
        stdin().read_line(&mut input).unwrap();
        let password = input.trim().to_string();
        config.users.push(User { username, password });
        let toml = toml::to_string(&config).unwrap();
        let mut file = std::fs::File::create(config_path).unwrap();
        file.write_all(toml.as_bytes()).unwrap();
        config
    } else {
        let toml = std::fs::read_to_string(config_path).unwrap();
        toml::from_str(&toml).unwrap()
    }
}

use axum::{extract::Multipart, routing::post, Router};

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

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let config = read_config();
    let app = Router::new().route("/upload", post(upload));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
