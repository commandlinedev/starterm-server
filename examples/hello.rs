#![deny(warnings)]
use starterm::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = starterm::any().map(|| "Hello, World!");

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
