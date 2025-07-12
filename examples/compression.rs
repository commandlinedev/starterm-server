#![deny(warnings)]

use starterm::Filter;

#[tokio::main]
async fn main() {
    let file = starterm::path("todos").and(starterm::fs::file("./examples/todos.rs"));
    // NOTE: You could double compress something by adding a compression
    // filter here, a la
    // ```
    // let file = starterm::path("todos")
    //     .and(starterm::fs::file("./examples/todos.rs"))
    //     .with(starterm::compression::brotli());
    // ```
    // This would result in a browser error, or downloading a file whose contents
    // are compressed

    let dir = starterm::path("ws_chat").and(starterm::fs::file("./examples/websockets_chat.rs"));

    let file_and_dir = starterm::get()
        .and(file.or(dir))
        .with(starterm::compression::gzip());

    let examples = starterm::path("ex")
        .and(starterm::fs::dir("./examples/"))
        .with(starterm::compression::deflate());

    // GET /todos => gzip -> toods.rs
    // GET /ws_chat => gzip -> ws_chat.rs
    // GET /ex/... => deflate -> ./examples/...
    let routes = file_and_dir.or(examples);

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
