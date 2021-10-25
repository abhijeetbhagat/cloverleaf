use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::ssl::{SslContextBuilder, SslMethod, SslVerifyMode};
use openssl::x509::X509;
use srtp2_sys::*;

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
        ctx_builder
            .set_tlsext_use_srtp("SRTP_AES128_CM_SHA1_80:SRTP_AES128_CM_SHA1_32")
            .map_err(|_| String::from("couldn't create an ssl context builder"))?;

        match (std::fs::read(cert_path), std::fs::read(key_path)) {
            (Ok(cert), Ok(key)) => {
                let cert = X509::from_pem(&cert).unwrap();
                let key = PKey::private_key_from_pem(&key).unwrap();
                ctx_builder.set_certificate(&cert);
                ctx_builder.set_private_key(&key);
                if let Ok(_) = ctx_builder.check_private_key() {
                    ctx_builder.set_read_ahead(true);
                    let digest_bytes = cert.digest(MessageDigest::sha256());
                    ctx_builder.set_cipher_list(
                        "DEFAULT:!NULL:!aNULL:!SHA256:!SHA384:!aECDH:!AESGCM+AES256:!aPSK",
                    );
                    unsafe {
                        srtp_init();
                    }
                }
            }
            _ => {}
        }

        todo!()
    }
}