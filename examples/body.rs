#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};

use starterm::Filter;

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = starterm::post()
        .and(starterm::path("employees"))
        .and(starterm::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(starterm::body::content_length_limit(1024 * 16))
        .and(starterm::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            starterm::reply::json(&employee)
        });

    starterm::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
