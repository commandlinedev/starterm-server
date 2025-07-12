#![deny(warnings)]
#[macro_use]
extern crate starterm;

use futures_util::future;
use starterm::Filter;

#[tokio::test]
async fn path() {
    let _ = pretty_env_logger::try_init();

    let foo = starterm::path("foo");
    let bar = starterm::path(String::from("bar"));
    let foo_bar = foo.and(bar.clone());

    // /foo
    let foo_req = || starterm::test::request().path("/foo");

    assert!(foo_req().matches(&foo).await);
    assert!(!foo_req().matches(&bar).await);
    assert!(!foo_req().matches(&foo_bar).await);

    // /foo/bar
    let foo_bar_req = || starterm::test::request().path("/foo/bar");

    assert!(foo_bar_req().matches(&foo).await);
    assert!(!foo_bar_req().matches(&bar).await);
    assert!(foo_bar_req().matches(&foo_bar).await);
}

#[tokio::test]
async fn param() {
    let _ = pretty_env_logger::try_init();

    let num = starterm::path::param::<u32>();

    let req = starterm::test::request().path("/321");
    assert_eq!(req.filter(&num).await.unwrap(), 321);

    let s = starterm::path::param::<String>();

    let req = starterm::test::request().path("/starterm");
    assert_eq!(req.filter(&s).await.unwrap(), "starterm");

    // u32 doesn't extract a non-int
    let req = starterm::test::request().path("/starterm");
    assert!(!req.matches(&num).await);

    let combo = num.map(|n| n + 5).and(s);

    let req = starterm::test::request().path("/42/vroom");
    assert_eq!(req.filter(&combo).await.unwrap(), (47, "vroom".to_string()));

    // empty segments never match
    let req = starterm::test::request();
    assert!(
        !req.matches(&s).await,
        "param should never match an empty segment"
    );
}

#[tokio::test]
async fn end() {
    let _ = pretty_env_logger::try_init();

    let foo = starterm::path("foo");
    let end = starterm::path::end();
    let foo_end = foo.and(end);

    assert!(
        starterm::test::request().path("/").matches(&end).await,
        "end() matches /"
    );

    assert!(
        starterm::test::request()
            .path("http://localhost:1234")
            .matches(&end)
            .await,
        "end() matches /"
    );

    assert!(
        starterm::test::request()
            .path("http://localhost:1234?q=2")
            .matches(&end)
            .await,
        "end() matches empty path"
    );

    assert!(
        starterm::test::request()
            .path("localhost:1234")
            .matches(&end)
            .await,
        "end() matches authority-form"
    );

    assert!(
        !starterm::test::request().path("/foo").matches(&end).await,
        "end() doesn't match /foo"
    );

    assert!(
        starterm::test::request()
            .path("/foo")
            .matches(&foo_end)
            .await,
        "path().and(end()) matches /foo"
    );

    assert!(
        starterm::test::request()
            .path("/foo/")
            .matches(&foo_end)
            .await,
        "path().and(end()) matches /foo/"
    );
}

#[tokio::test]
async fn tail() {
    let tail = starterm::path::tail();

    // matches full path
    let ex = starterm::test::request()
        .path("/42/vroom")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "42/vroom");

    // matches index
    let ex = starterm::test::request()
        .path("/")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "");

    // doesn't include query
    let ex = starterm::test::request()
        .path("/foo/bar?baz=quux")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "foo/bar");

    // doesn't include previously matched prefix
    let and = starterm::path("foo").and(tail);
    let ex = starterm::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "bar");

    // sets unmatched path index to end
    let m = tail.and(starterm::path("foo"));
    assert!(!starterm::test::request().path("/foo/bar").matches(&m).await);

    let m = tail.and(starterm::path::end());
    assert!(starterm::test::request().path("/foo/bar").matches(&m).await);

    let ex = starterm::test::request()
        .path("localhost")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/");
}

#[tokio::test]
async fn or() {
    let _ = pretty_env_logger::try_init();

    // /foo/bar OR /foo/baz
    let foo = starterm::path("foo");
    let bar = starterm::path("bar");
    let baz = starterm::path("baz");
    let p = foo.and(bar.or(baz));

    // /foo/bar
    let req = starterm::test::request().path("/foo/bar");

    assert!(req.matches(&p).await);

    // /foo/baz
    let req = starterm::test::request().path("/foo/baz");

    assert!(req.matches(&p).await);

    // deeper nested ORs
    // /foo/bar/baz OR /foo/baz/bar OR /foo/bar/bar
    let p = foo
        .and(bar.and(baz).map(|| panic!("shouldn't match")))
        .or(foo.and(baz.and(bar)).map(|| panic!("shouldn't match")))
        .or(foo.and(bar.and(bar)));

    // /foo/baz
    let req = starterm::test::request().path("/foo/baz/baz");
    assert!(!req.matches(&p).await);

    // /foo/bar/bar
    let req = starterm::test::request().path("/foo/bar/bar");
    assert!(req.matches(&p).await);
}

#[tokio::test]
async fn or_else() {
    let _ = pretty_env_logger::try_init();

    let foo = starterm::path("foo");
    let bar = starterm::path("bar");

    let p = foo.and(bar.or_else(|_| future::ok::<_, std::convert::Infallible>(())));

    // /foo/bar
    let req = starterm::test::request().path("/foo/nope");

    assert!(req.matches(&p).await);
}

#[tokio::test]
async fn path_macro() {
    let _ = pretty_env_logger::try_init();

    let req = starterm::test::request().path("/foo/bar");
    let p = path!("foo" / "bar");
    assert!(req.matches(&p).await);

    let req = starterm::test::request().path("/foo/bar");
    let p = path!(String / "bar");
    assert_eq!(req.filter(&p).await.unwrap(), "foo");

    let req = starterm::test::request().path("/foo/bar");
    let p = path!("foo" / String);
    assert_eq!(req.filter(&p).await.unwrap(), "bar");

    // Requires path end

    let req = starterm::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar");
    assert!(!req.matches(&p).await);

    let req = starterm::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar").and(starterm::path("baz"));
    assert!(!req.matches(&p).await);

    // Prefix syntax

    let req = starterm::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar" / ..);
    assert!(req.matches(&p).await);

    let req = starterm::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar" / ..).and(starterm::path!("baz"));
    assert!(req.matches(&p).await);

    // Empty

    let req = starterm::test::request().path("/");
    let p = path!();
    assert!(req.matches(&p).await);

    let req = starterm::test::request().path("/foo");
    let p = path!();
    assert!(!req.matches(&p).await);
}

#[tokio::test]
async fn full_path() {
    let full_path = starterm::path::full();

    let foo = starterm::path("foo");
    let bar = starterm::path("bar");
    let param = starterm::path::param::<u32>();

    // matches full request path
    let ex = starterm::test::request()
        .path("/42/vroom")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/42/vroom");

    // matches index
    let ex = starterm::test::request()
        .path("/")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/");

    // does not include query
    let ex = starterm::test::request()
        .path("/foo/bar?baz=quux")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/bar");

    // includes previously matched prefix
    let and = foo.and(full_path);
    let ex = starterm::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/bar");

    // includes following matches
    let and = full_path.and(foo);
    let ex = starterm::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/bar");

    // includes previously matched param
    let and = foo.and(param).and(full_path);
    let (_, ex) = starterm::test::request()
        .path("/foo/123")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/123");

    // does not modify matching
    let m = full_path.and(foo).and(bar);
    assert!(starterm::test::request().path("/foo/bar").matches(&m).await);

    // doesn't panic on authority-form
    let ex = starterm::test::request()
        .path("localhost:1234")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/");
}

#[tokio::test]
async fn peek() {
    let peek = starterm::path::peek();

    let foo = starterm::path("foo");
    let bar = starterm::path("bar");
    let param = starterm::path::param::<u32>();

    // matches full request path
    let ex = starterm::test::request()
        .path("/42/vroom")
        .filter(&peek)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "42/vroom");

    // matches index
    let ex = starterm::test::request()
        .path("/")
        .filter(&peek)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "");

    // does not include query
    let ex = starterm::test::request()
        .path("/foo/bar?baz=quux")
        .filter(&peek)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "foo/bar");

    // does not include previously matched prefix
    let and = foo.and(peek);
    let ex = starterm::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "bar");

    // includes following matches
    let and = peek.and(foo);
    let ex = starterm::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "foo/bar");

    // does not include previously matched param
    let and = foo.and(param).and(peek);
    let (_, ex) = starterm::test::request()
        .path("/foo/123")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "");

    // does not modify matching
    let and = peek.and(foo).and(bar);
    assert!(
        starterm::test::request()
            .path("/foo/bar")
            .matches(&and)
            .await
    );
}

#[tokio::test]
async fn peek_segments() {
    let peek = starterm::path::peek();

    // matches full request path
    let ex = starterm::test::request()
        .path("/42/vroom")
        .filter(&peek)
        .await
        .unwrap();

    assert_eq!(ex.segments().collect::<Vec<_>>(), &["42", "vroom"]);

    // matches index
    let ex = starterm::test::request()
        .path("/")
        .filter(&peek)
        .await
        .unwrap();

    let segs = ex.segments().collect::<Vec<_>>();
    assert_eq!(segs, Vec::<&str>::new());
}
