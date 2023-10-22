use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
    pub refresh_encoding: EncodingKey,
    pub refresh_decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8], refresh_secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
            refresh_encoding: EncodingKey::from_secret(refresh_secret),
            refresh_decoding: DecodingKey::from_secret(refresh_secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let refresh_secret =
        std::env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    Keys::new(secret.as_bytes(), refresh_secret.as_bytes())
});
