#![deny(warnings)]
use starterm::{http::Uri, Filter};

#[tokio::test]
async fn redirect_uri() {
    let over_there = starterm::any().map(|| starterm::redirect(Uri::from_static("/over-there")));

    let req = starterm::test::request();
    let resp = req.reply(&over_there).await;

    assert_eq!(resp.status(), 301);
    assert_eq!(resp.headers()["location"], "/over-there");
}

#[tokio::test]
async fn redirect_found_uri() {
    let over_there =
        starterm::any().map(|| starterm::redirect::found(Uri::from_static("/over-there")));

    let req = starterm::test::request();
    let resp = req.reply(&over_there).await;

    assert_eq!(resp.status(), 302);
    assert_eq!(resp.headers()["location"], "/over-there");
}

#[tokio::test]
async fn redirect_see_other_uri() {
    let over_there =
        starterm::any().map(|| starterm::redirect::see_other(Uri::from_static("/over-there")));

    let req = starterm::test::request();
    let resp = req.reply(&over_there).await;

    assert_eq!(resp.status(), 303);
    assert_eq!(resp.headers()["location"], "/over-there");
}

#[tokio::test]
async fn redirect_temporary_uri() {
    let over_there =
        starterm::any().map(|| starterm::redirect::temporary(Uri::from_static("/over-there")));

    let req = starterm::test::request();
    let resp = req.reply(&over_there).await;

    assert_eq!(resp.status(), 307);
    assert_eq!(resp.headers()["location"], "/over-there");
}

#[tokio::test]
async fn redirect_permanent_uri() {
    let over_there =
        starterm::any().map(|| starterm::redirect::permanent(Uri::from_static("/over-there")));

    let req = starterm::test::request();
    let resp = req.reply(&over_there).await;

    assert_eq!(resp.status(), 308);
    assert_eq!(resp.headers()["location"], "/over-there");
}
