use rocket::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]              // <- route attribute
pub fn get_all_users() -> &'static str {  // <- request handler
    "Get all Users!!"
}

#[get("/world")]              // <- route attribute
pub fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

use rocket::tokio::time::{sleep, Duration};

#[get("/users/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}