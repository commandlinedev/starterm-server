#![deny(warnings)]

use starterm::addr::remote;
use starterm::test::request;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Ensures that extracting remote address returns None when no remote address is set.
#[tokio::test]
async fn remote_addr_missing() {
    let extract_remote_addr = remote();

    let req = request();
    let resp = req.filter(&extract_remote_addr).await.unwrap();
    assert_eq!(
        resp, None,
        "Expected None when no remote address is set, got {:?}",
        resp
    );
}

/// Ensures that extracting remote address returns the correct address when set.
#[tokio::test]
async fn remote_addr_present() {
    let extract_remote_addr = remote();

    let req = request().remote_addr("1.2.3.4:5678".parse().unwrap());
    let resp = req.filter(&extract_remote_addr).await.unwrap();
    let expected = Some(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 5678));
    assert_eq!(resp, expected, "Expected {:?}, got {:?}", expected, resp);
}
