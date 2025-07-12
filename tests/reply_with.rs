#![deny(warnings)]
use starterm::http::header::{HeaderMap, HeaderValue};
use starterm::Filter;

#[tokio::test]
async fn header() {
    let header = starterm::reply::with::header("foo", "bar");

    let no_header = starterm::any().map(starterm::reply).with(&header);

    let req = starterm::test::request();
    let resp = req.reply(&no_header).await;
    assert_eq!(resp.headers()["foo"], "bar");

    let prev_header = starterm::reply::with::header("foo", "sean");
    let yes_header = starterm::any()
        .map(starterm::reply)
        .with(prev_header)
        .with(header);

    let req = starterm::test::request();
    let resp = req.reply(&yes_header).await;
    assert_eq!(resp.headers()["foo"], "bar", "replaces header");
}

#[tokio::test]
async fn headers() {
    let mut headers = HeaderMap::new();
    headers.insert("server", HeaderValue::from_static("starterm"));
    headers.insert("foo", HeaderValue::from_static("bar"));

    let headers = starterm::reply::with::headers(headers);

    let no_header = starterm::any().map(starterm::reply).with(&headers);

    let req = starterm::test::request();
    let resp = req.reply(&no_header).await;
    assert_eq!(resp.headers()["foo"], "bar");
    assert_eq!(resp.headers()["server"], "starterm");

    let prev_header = starterm::reply::with::header("foo", "sean");
    let yes_header = starterm::any()
        .map(starterm::reply)
        .with(prev_header)
        .with(headers);

    let req = starterm::test::request();
    let resp = req.reply(&yes_header).await;
    assert_eq!(resp.headers()["foo"], "bar", "replaces header");
}

#[tokio::test]
async fn default_header() {
    let def_header = starterm::reply::with::default_header("foo", "bar");

    let no_header = starterm::any().map(starterm::reply).with(&def_header);

    let req = starterm::test::request();
    let resp = req.reply(&no_header).await;

    assert_eq!(resp.headers()["foo"], "bar");

    let header = starterm::reply::with::header("foo", "sean");
    let yes_header = starterm::any()
        .map(starterm::reply)
        .with(header)
        .with(def_header);

    let req = starterm::test::request();
    let resp = req.reply(&yes_header).await;

    assert_eq!(resp.headers()["foo"], "sean", "doesn't replace header");
}
