use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{delete, get, post, routes, State};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMessage {
    message: String,
}

#[get("/get/<key>")]
fn get_value(db: &State<Mutex<HashMap<String, String>>>, key: String) -> Option<Json<KeyValue>> {
    let db = db.lock().unwrap();
    db.get(&key).map(|value| {
        Json(KeyValue {
            key: key.clone(),
            value: value.clone(),
        })
    })
}

#[post("/set", format = "json", data = "<key_value>")]
fn set_value(db: &State<Mutex<HashMap<String, String>>>, key_value: Json<KeyValue>) -> Json<ResponseMessage> {
    let mut db = db.lock().unwrap();
    db.insert(key_value.key.clone(), key_value.value.clone());
    Json(ResponseMessage {
        message: format!("Key '{}' set successfully.", key_value.key),
    })
}

#[delete("/delete/<key>")]
fn delete_value(db: &State<Mutex<HashMap<String, String>>>, key: String) -> Json<ResponseMessage> {
    let mut db = db.lock().unwrap();
    match db.remove(&key) {
        Some(_) => Json(ResponseMessage {
            message: format!("Key '{}' deleted successfully.", key),
        }),
        None => Json(ResponseMessage {
            message: format!("Key '{}' not found.", key),
        }),
    }
}

#[post("/clear")]
fn clear_db(db: &State<Mutex<HashMap<String, String>>>) -> Json<ResponseMessage> {
    let mut db = db.lock().unwrap();
    db.clear();
    Json(ResponseMessage {
        message: String::from("Database cleared."),
    })
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .mount("/", routes![get_value, set_value, delete_value, clear_db]);
    Ok(rocket.into())
}