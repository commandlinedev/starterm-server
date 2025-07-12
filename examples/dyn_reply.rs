#![deny(warnings)]
use starterm::{http::StatusCode, Filter};

async fn dyn_reply(word: String) -> Result<Box<dyn starterm::Reply>, starterm::Rejection> {
    if &word == "hello" {
        Ok(Box::new("world"))
    } else {
        Ok(Box::new(StatusCode::BAD_REQUEST))
    }
}

#[tokio::main]
async fn main() {
    let routes = starterm::path::param().and_then(dyn_reply);

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
