#![deny(warnings)]

// Don't copy this `cfg`, it's only needed because this file is within
// the starterm repository.
// Instead, specify the "tls" feature in your starterm dependency declaration.
#[cfg(feature = "tls")]
#[tokio::main]
async fn main() {
    use starterm::Filter;

    // Match any request and return hello world!
    let routes = starterm::any().map(|| "Hello, World!");

    starterm::serve(routes)
        .tls()
        // RSA
        .cert_path("examples/tls/cert.pem")
        .key_path("examples/tls/key.rsa")
        // ECC
        // .cert_path("examples/tls/cert.ecc.pem")
        // .key_path("examples/tls/key.ecc")
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[cfg(not(feature = "tls"))]
fn main() {
    eprintln!("Requires the `tls` feature.");
}
