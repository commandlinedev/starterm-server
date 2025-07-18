//! [`tracing`] is a framework for instrumenting Rust programs to
//! collect scoped, structured, and async-aware diagnostics. This example
//! demonstrates how the `starterm::trace` module can be used to instrument `starterm`
//! applications with `tracing`.
//!
//! [`tracing`]: https://crates.io/crates/tracing
#![deny(warnings)]
use starterm::Filter;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    // Filter traces based on the RUST_LOG env var, or, if it's not set,
    // default to show the output of the example.
    let filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,starterm=debug".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let hello = starterm::path("hello")
        .and(starterm::get())
        // When the `hello` route is called, emit a `tracing` event.
        .map(|| {
            tracing::info!("saying hello...");
            "Hello, World!"
        })
        // Wrap the route in a `tracing` span to add the route's name as context
        // to any events that occur inside it.
        .with(starterm::trace::named("hello"));

    let goodbye = starterm::path("goodbye")
        .and(starterm::get())
        .map(|| {
            tracing::info!("saying goodbye...");
            "So long and thanks for all the fish!"
        })
        // We can also provide our own custom `tracing` spans to wrap a route.
        .with(starterm::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("goodbye", req.path = ?info.path())
        }));

    let routes = hello
        .or(goodbye)
        // Wrap all the routes with a filter that creates a `tracing` span for
        // each request we receive, including data about the request.
        .with(starterm::trace::request());

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
