#![deny(warnings)]

use starterm::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let readme = starterm::get()
        .and(starterm::path::end())
        .and(starterm::fs::file("./README.md"));

    // dir already requires GET...
    let examples = starterm::path("ex").and(starterm::fs::dir("./examples/"));

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = readme.or(examples);

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
