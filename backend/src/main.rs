#[macro_use] extern crate rocket;
use std::path::PathBuf;
use rocket::{fs::NamedFile, response::status::NotFound};
use common::Markup;
use std::fs;
use rocket::serde::json::Json;
mod database_manager;

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/markup/<path..>")]
fn get_markup(path: PathBuf) -> Json<Markup> {
    let path = PathBuf::from("data/markup/").join(path);
    match fs::read_to_string(path) {
        Ok(markup) => Json(Markup {
            markup: String::from(markup),
        }),
        Err(e) => Json(Markup {
            markup: String::from(format!("Error: {}", e)),
        }),
    }
}

//Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../ui/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f), 
        Err(_) => get_index().await,
    }
}

#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./data/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[launch]
async fn rocket() -> _ {

    let db = database_manager::connect().await;

    rocket::build()
        .mount("/", routes![index, static_files, data])
        .mount("/api", routes![get_markup])
}