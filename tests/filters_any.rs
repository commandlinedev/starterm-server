//! Tests for the `any` filter.

#[tokio::test]
async fn test_any_filter_matches_any_request() {
    let filter = starterm::any();
    let result = starterm::test::request().filter(&filter).await;
    assert!(result.is_ok());
}
