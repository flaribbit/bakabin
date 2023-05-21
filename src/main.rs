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
    // check if `config.toml` exists
    // ask for username and password
    // then store it in `config.toml`
    if !Path::new("config.toml").exists() {
        let mut config = Config { users: Vec::new() };
        let mut input = String::new();
        println!("No config file found. Creating one...");
        println!("Enter username: ");
        stdin().read_line(&mut input).unwrap();
        let username = input.trim().to_string();
        input.clear();
        println!("Enter password: ");
        stdin().read_line(&mut input).unwrap();
        let password = input.trim().to_string();
        config.users.push(User { username, password });
        let toml = toml::to_string(&config).unwrap();
        let mut file = std::fs::File::create("config.toml").unwrap();
        file.write_all(toml.as_bytes()).unwrap();
        config
    } else {
        let toml = std::fs::read_to_string("config.toml").unwrap();
        toml::from_str(&toml).unwrap()
    }
}

fn main() {
    let config = read_config();
    println!("{:?}", config);
    println!("Hello, world!");
}
