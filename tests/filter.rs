#![deny(warnings)]
use starterm::Filter;
use std::convert::Infallible;

#[tokio::test]
async fn flattens_tuples() {
    let _ = pretty_env_logger::try_init();

    let str1 = starterm::any().map(|| "starterm");
    let true1 = starterm::any().map(|| true);
    let unit1 = starterm::any();

    // just 1 value
    let ext = starterm::test::request().filter(&str1).await.unwrap();
    assert_eq!(ext, "starterm");

    // just 1 unit
    let ext = starterm::test::request().filter(&unit1).await.unwrap();
    assert_eq!(ext, ());

    // combine 2 values
    let and = str1.and(true1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", true));

    // combine 2 reversed
    let and = true1.and(str1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, (true, "starterm"));

    // combine 1 with unit
    let and = str1.and(unit1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, "starterm");

    let and = unit1.and(str1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, "starterm");

    // combine 3 values
    let and = str1.and(str1).and(true1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", "starterm", true));

    // combine 2 with unit
    let and = str1.and(unit1).and(true1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", true));

    let and = unit1.and(str1).and(true1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", true));

    let and = str1.and(true1).and(unit1);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", true));

    // nested tuples
    let str_true_unit = str1.and(true1).and(unit1);
    let unit_str_true = unit1.and(str1).and(true1);

    let and = str_true_unit.and(unit_str_true);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", true, "starterm", true));

    let and = unit_str_true.and(unit1).and(str1).and(str_true_unit);
    let ext = starterm::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("starterm", true, "starterm", "starterm", true));
}

#[tokio::test]
async fn map() {
    let _ = pretty_env_logger::try_init();

    let ok = starterm::any().map(starterm::reply);

    let req = starterm::test::request();
    let resp = req.reply(&ok).await;
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn or() {
    let _ = pretty_env_logger::try_init();

    // Or can be combined with an infallible filter
    let a = starterm::path::param::<u32>();
    let b = starterm::any().map(|| 41i32);
    let f = a.or(b);

    let _: Result<_, Infallible> = starterm::test::request().filter(&f).await;
}

#[tokio::test]
async fn or_else() {
    let _ = pretty_env_logger::try_init();

    let a = starterm::path::param::<u32>();
    let f = a.or_else(|_| async { Ok::<_, starterm::Rejection>((44u32,)) });

    assert_eq!(
        starterm::test::request()
            .path("/33")
            .filter(&f)
            .await
            .unwrap(),
        33,
    );
    assert_eq!(starterm::test::request().filter(&f).await.unwrap(), 44,);

    // OrElse can be combined with an infallible filter
    let a = starterm::path::param::<u32>();
    let f = a.or_else(|_| async { Ok::<_, Infallible>((44u32,)) });

    let _: Result<_, Infallible> = starterm::test::request().filter(&f).await;
}

#[tokio::test]
async fn recover() {
    let _ = pretty_env_logger::try_init();

    let a = starterm::path::param::<String>();
    let f = a.recover(|err| async move { Err::<String, _>(err) });

    // not rejected
    let resp = starterm::test::request().path("/hi").reply(&f).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.body(), "hi");

    // rejected, recovered, re-rejected
    let resp = starterm::test::request().reply(&f).await;
    assert_eq!(resp.status(), 404);

    // Recover can be infallible
    let f = a.recover(|_| async move { Ok::<_, Infallible>("shh") });

    let _: Result<_, Infallible> = starterm::test::request().filter(&f).await;
}

#[tokio::test]
async fn unify() {
    let _ = pretty_env_logger::try_init();

    let a = starterm::path::param::<u32>();
    let b = starterm::path::param::<u32>();
    let f = a.or(b).unify();

    let ex = starterm::test::request()
        .path("/1")
        .filter(&f)
        .await
        .unwrap();

    assert_eq!(ex, 1);
}

#[should_panic]
#[tokio::test]
async fn nested() {
    let f = starterm::any().and_then(|| async {
        let p = starterm::path::param::<u32>();
        starterm::test::request().filter(&p).await
    });

    let _ = starterm::test::request().filter(&f).await;
}
