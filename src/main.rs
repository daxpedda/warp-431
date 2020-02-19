use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    let (certificate, private_key) = if let Ok(certificate) = fs::read("target/certificate.pem") {
        (
            String::from_utf8(certificate).unwrap(),
            String::from_utf8(fs::read("target/private_key.pem").unwrap()).unwrap(),
        )
    } else {
        let cert_pair = rcgen::generate_simple_self_signed(Vec::new())
            .expect("failed to build certificate pair");

        let certificate = cert_pair
            .serialize_pem()
            .expect("failed to serialize certificate");
        let private_key = cert_pair.serialize_private_key_pem();

        fs::write("target/certificate.pem", &certificate).unwrap();
        fs::write("target/private_key.pem", &private_key).unwrap();

        (certificate, private_key)
    };

    warp::serve(warp::path!("hello" / String).map(|name| format!("Hello, {}!", name)))
        .tls()
        .cert(certificate)
        .key(private_key)
        .upgrade(true)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
