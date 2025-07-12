use serde_derive::{Deserialize, Serialize};
use starterm::{
    http::{Response, StatusCode},
    Filter,
};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct MyObject {
    key1: String,
    key2: u32,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // get /example1?key=value
    // demonstrates an optional parameter.
    let example1 = starterm::get()
        .and(starterm::path("example1"))
        .and(starterm::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("key") {
            Some(key) => Response::builder().body(format!("key = {}", key)),
            None => Response::builder().body(String::from("No \"key\" param in query.")),
        });

    // get /example2?key1=value&key2=42
    // uses the query string to populate a custom object
    let example2 = starterm::get()
        .and(starterm::path("example2"))
        .and(starterm::query::<MyObject>())
        .map(|p: MyObject| {
            Response::builder().body(format!("key1 = {}, key2 = {}", p.key1, p.key2))
        });

    let opt_query = starterm::query::<MyObject>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<MyObject>,), std::convert::Infallible>((None,)) });

    // get /example3?key1=value&key2=42
    // builds on example2 but adds custom error handling
    let example3 = starterm::get()
        .and(starterm::path("example3"))
        .and(opt_query)
        .map(|p: Option<MyObject>| match p {
            Some(obj) => {
                Response::builder().body(format!("key1 = {}, key2 = {}", obj.key1, obj.key2))
            }
            None => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(String::from("Failed to decode query param.")),
        });

    starterm::serve(example1.or(example2).or(example3))
        .run(([127, 0, 0, 1], 3030))
        .await
}
