#![deny(warnings)]

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    starterm::serve(starterm::fs::dir("examples/dir"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
