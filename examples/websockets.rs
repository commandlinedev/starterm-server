#![deny(warnings)]

use futures_util::{FutureExt, StreamExt};
use starterm::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = starterm::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(starterm::ws())
        .map(|ws: starterm::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    starterm::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
