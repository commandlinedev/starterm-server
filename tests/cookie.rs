#![deny(warnings)]

#[tokio::test]
async fn cookie() {
    let foo = starterm::cookie::<String>("foo");

    let req = starterm::test::request().header("cookie", "foo=bar");
    assert_eq!(req.filter(&foo).await.unwrap(), "bar");

    let req = starterm::test::request().header("cookie", "abc=def; foo=baz");
    assert_eq!(req.filter(&foo).await.unwrap(), "baz");

    let req = starterm::test::request().header("cookie", "abc=def");
    assert!(!req.matches(&foo).await);

    let req = starterm::test::request().header("cookie", "foobar=quux");
    assert!(!req.matches(&foo).await);
}

#[tokio::test]
async fn optional() {
    let foo = starterm::cookie::optional::<String>("foo");

    let req = starterm::test::request().header("cookie", "foo=bar");
    assert_eq!(req.filter(&foo).await.unwrap().unwrap(), "bar");

    let req = starterm::test::request().header("cookie", "abc=def; foo=baz");
    assert_eq!(req.filter(&foo).await.unwrap().unwrap(), "baz");

    let req = starterm::test::request().header("cookie", "abc=def");
    assert!(req.matches(&foo).await);

    let req = starterm::test::request().header("cookie", "foobar=quux");
    assert!(req.matches(&foo).await);
}

#[tokio::test]
async fn missing() {
    let _ = pretty_env_logger::try_init();

    let cookie = starterm::cookie::<String>("foo");

    let res = starterm::test::request()
        .header("cookie", "not=here")
        .reply(&cookie)
        .await;

    assert_eq!(res.status(), 400);
    assert_eq!(res.body(), "Missing request cookie \"foo\"");
}
