use futures_util::StreamExt;
use starterm::{sse::Event, Filter};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

// create server-sent event
fn sse_counter(counter: u64) -> Result<Event, Infallible> {
    Ok(starterm::sse::Event::default().data(counter.to_string()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = starterm::path("ticks").and(starterm::get()).map(|| {
        let mut counter: u64 = 0;
        // create server event source
        let interval = interval(Duration::from_secs(1));
        let stream = IntervalStream::new(interval);
        let event_stream = stream.map(move |_| {
            counter += 1;
            sse_counter(counter)
        });
        // reply using server-sent events
        starterm::sse::reply(event_stream)
    });

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
