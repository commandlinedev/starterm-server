#![deny(warnings)]

#[cfg(unix)]
#[tokio::main]
async fn main() {
    use tokio::net::UnixListener;
    use tokio_stream::wrappers::UnixListenerStream;

    pretty_env_logger::init();

    let listener = UnixListener::bind("/tmp/starterm.sock").unwrap();
    let incoming = UnixListenerStream::new(listener);
    starterm::serve(starterm::fs::dir("examples/dir"))
        .run_incoming(incoming)
        .await;
}

#[cfg(not(unix))]
#[tokio::main]
async fn main() {
    panic!("Must run under Unix-like platform!");
}
