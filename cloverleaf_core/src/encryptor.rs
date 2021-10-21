use openssl::pkey::PKey;
use openssl::ssl::{SslContextBuilder, SslMethod, SslVerifyMode};
use openssl::x509::X509;

pub struct Encryptor {}

impl Encryptor {
    pub fn new(cert_path: &str, key_path: &str) -> Result<Encryptor, String> {
        let method = SslMethod::dtls();
        let mut ctx_builder = SslContextBuilder::new(method)
            .map_err(|_| String::from("couldn't create an ssl context builder"))?;
        ctx_builder.set_verify_callback(
            SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT,
            |preverify, x509_store_ctx_ref| true,
        );

        match (std::fs::read(cert_path), std::fs::read(key_path)) {
            (Ok(cert), Ok(key)) => {
                let cert = X509::from_pem(&cert);
                let key = PKey::private_key_from_pem(&key);
            }
            _ => {}
        }

        todo!()
    }
}
