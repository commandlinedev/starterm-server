#![deny(warnings)]
use starterm::host::Authority;

#[tokio::test]
async fn exact() {
    let filter = starterm::host::exact("known.com");

    // no authority
    let req = starterm::test::request();
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in URI
    let req = starterm::test::request().path("http://known.com/about-us");
    assert!(req.filter(&filter).await.is_ok());

    let req = starterm::test::request().path("http://unknown.com/about-us");
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in Host header
    let req = starterm::test::request()
        .header("host", "known.com")
        .path("/about-us");
    assert!(req.filter(&filter).await.is_ok());

    let req = starterm::test::request()
        .header("host", "unknown.com")
        .path("/about-us");
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in both - matching
    let req = starterm::test::request()
        .header("host", "known.com")
        .path("http://known.com/about-us");
    assert!(req.filter(&filter).await.is_ok());

    let req = starterm::test::request()
        .header("host", "unknown.com")
        .path("http://unknown.com/about-us");
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in both - mismatch
    let req = starterm::test::request()
        .header("host", "known.com")
        .path("http://known2.com/about-us");
    assert!(req
        .filter(&filter)
        .await
        .unwrap_err()
        .find::<starterm::reject::InvalidHeader>()
        .is_some());

    // bad host header - invalid chars
    let req = starterm::test::request()
        .header("host", "😭")
        .path("http://known.com/about-us");
    assert!(req
        .filter(&filter)
        .await
        .unwrap_err()
        .find::<starterm::reject::InvalidHeader>()
        .is_some());

    // bad host header - invalid format
    let req = starterm::test::request()
        .header("host", "hello space.com")
        .path("http://known.com//about-us");
    assert!(req
        .filter(&filter)
        .await
        .unwrap_err()
        .find::<starterm::reject::InvalidHeader>()
        .is_some());
}

#[tokio::test]
async fn optional() {
    let filter = starterm::host::optional();

    let req = starterm::test::request().header("host", "example.com");
    assert_eq!(
        req.filter(&filter).await.unwrap(),
        Some(Authority::from_static("example.com"))
    );

    let req = starterm::test::request();
    assert_eq!(req.filter(&filter).await.unwrap(), None);
}
