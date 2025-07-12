#![deny(warnings)]
use starterm::Filter;

#[tokio::test]
async fn exact() {
    let _ = pretty_env_logger::try_init();

    let host = starterm::header::exact("host", "localhost");

    let req = starterm::test::request().header("host", "localhost");

    assert!(req.matches(&host).await);

    let req = starterm::test::request();
    assert!(!req.matches(&host).await, "header missing");

    let req = starterm::test::request().header("host", "hyper.rs");
    assert!(!req.matches(&host).await, "header value different");
}

#[tokio::test]
async fn exact_rejections() {
    let _ = pretty_env_logger::try_init();

    let host = starterm::header::exact("host", "localhost").map(starterm::reply);

    let res = starterm::test::request()
        .header("host", "nope")
        .reply(&host)
        .await;

    assert_eq!(res.status(), 400);
    assert_eq!(res.body(), "Invalid request header \"host\"");

    let res = starterm::test::request()
        .header("not-even-a-host", "localhost")
        .reply(&host)
        .await;

    assert_eq!(res.status(), 400);
    assert_eq!(res.body(), "Missing request header \"host\"");
}

#[tokio::test]
async fn optional() {
    let _ = pretty_env_logger::try_init();

    let con_len = starterm::header::optional::<u64>("content-length");

    let val = starterm::test::request()
        .filter(&con_len)
        .await
        .expect("missing header matches");
    assert_eq!(val, None);

    let val = starterm::test::request()
        .header("content-length", "5")
        .filter(&con_len)
        .await
        .expect("existing header matches");

    assert_eq!(val, Some(5));

    assert!(
        !starterm::test::request()
            .header("content-length", "boom")
            .matches(&con_len)
            .await,
        "invalid optional header still rejects",
    );
}
