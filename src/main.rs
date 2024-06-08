use axum::{routing::post, Router, Json};
use serde::Deserialize;
use std::io::Read;
use sqlite;
use toml;

#[derive(Deserialize)]
struct Note{
    ctx: String,
    user: String,
    password: String,
}

#[derive(Deserialize)]
struct Auth{
    user: String,
    password: String,
}
async fn list(Json(auth): Json<Auth>) -> Json<Vec<String>>{
    let mut data = String::new();
    std::fs::File::open("config").expect("Failed to open config").read_to_string(&mut data).expect("Failed to open config");
    let cfg: Auth = toml::from_str(&data).unwrap();
    if auth.user == cfg.user && auth.password == cfg.password{
        let con = sqlite::open("db.sql").unwrap();
        let query = "SELECT * FROM notes;";
        let mut response: Vec<String> = vec![];
        con.iterate(query, |pairs| {
            for &(_name, value) in pairs.iter(){
                response.push(value.unwrap().to_string());
            }
            true
        })
        .unwrap();
        Json(response)
    }
    else{
        let fail: Vec<String> = vec!["Authentication failed".to_string()];
        Json(fail)
    }
}

async fn create(Json(note): Json<Note>) -> String{
    let mut data = String::new();
    std::fs::File::open("config").expect("Failed to open config").read_to_string(&mut data).expect("Failed to open config");
    let cfg: Auth = toml::from_str(&data).unwrap();
    if note.user == cfg.user && note.password == cfg.password{
        let con = sqlite::open("db.sql").unwrap();
        let query = "INSERT INTO notes (id, ctx) VALUES (NULL,'".to_owned() + &note.ctx + "');";
        con.execute(query).unwrap();
        "Added a new note".to_string()
        
    }
    else {
        "Authentication failed".to_string()
    }
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/list", post(list))
        .route("/new", post(create));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
