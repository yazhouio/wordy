// 获取客户端账号和密码（密码有 sha256 混淆）
// 对比配置文件中的账号和密码
// 返回是否验证成功，并返回包含 name 和 id 和 token

use anyhow::{anyhow, Result};
use blake2::{Blake2b512, Digest};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};

pub mod jwt;

const ACCESS_TOKEN_EXPIRE: i64 = 60 * 60;
const REFRESH_TOKEN_EXPIRE: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Auth {
    pub name: String,
    pub id: u64,
    pub client_salt: String,
    pub server_salt: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JWTToken {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JWTData {
    pub name: String,
    pub id: u64,
    pub exp: i64,
}

pub fn add_salt(password: &str, salt: &str) -> Option<String> {
    let salt = salt.split('-').collect::<Vec<_>>();
    if salt.len() != 5 {
        return None;
    }
    Some(format!(
        "{}-{}-{}-{}-{}-{}",
        salt[0], salt[1], salt[2], salt[3], password, salt[4]
    ))
}

pub fn hash(msg: &str) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(msg.as_bytes());
    let res = hasher.finalize();
    hex::encode(res)
}

impl Auth {
    pub fn new(
        name: String,
        id: u64,
        password: String,
        client_salt: String,
        server_salt: String,
    ) -> Self {
        Self {
            name,
            id,
            password,
            client_salt,
            server_salt,
        }
    }

    pub fn new_by_name(name: String) -> Result<Self> {
        let db_name = std::env::var("CLIENT_NAME").expect("CLIENT_NAME not found");
        if name != db_name {
            return Err(anyhow!("user not found"));
        }

        let db_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not found");
        let db_password = std::env::var("CLIENT_PASSWORD").expect("CLIENT_PASSWORD not found");
        let db_client_salt = std::env::var("CLIENT_PASSWORD_SALT").expect("CLIENT_PASSWORD_SALT not found");
        let db_server_salt = std::env::var("SERVER_PASSWORD_SALT").expect("SERVER_PASSWORD_SALT not found");
        Ok(Self::new(
            name,
            db_id.parse::<u64>().unwrap(),
            db_password,
            db_client_salt,
            db_server_salt,
        ))
    }

    pub fn from_access_token(token: &str) -> Result<Self> {
        let token_data = decode::<JWTData>(token, &jwt::KEYS.decoding, &Validation::default())
            .map_err(|e| {
                tracing::error!("validate_token error: {:?}", e);
                anyhow!(e)
            })?;
        if token_data.claims.exp < chrono::Utc::now().timestamp() {
            return Err(anyhow!("access token expired"));
        };
        Auth::new_by_name(token_data.claims.name)
    }

    pub fn from_refresh_token(token: &str) -> Result<Self> {
        let token_data =
            decode::<JWTData>(token, &jwt::KEYS.refresh_decoding, &Validation::default()).map_err(
                |e| {
                    tracing::error!("validate_token error: {:?}", e);
                    anyhow!(e)
                },
            )?;
        if token_data.claims.exp < chrono::Utc::now().timestamp() {
            return Err(anyhow!("refresh token expired"));
        }
        Auth::new_by_name(token_data.claims.name)
    }

    pub fn refresh_access_token(&self) -> Result<JWTToken> {
        JWTToken::generate_token::<JWTData>(&self.to_login_response())
    }

    pub fn generate_token(&self) -> Result<JWTToken> {
        JWTToken::generate_token::<JWTData>(&self.to_login_response())
    }

    pub fn to_login_response(&self) -> (JWTData, JWTData) {
        (
            JWTData {
                name: self.name.clone(),
                id: self.id,
                exp: chrono::Utc::now().timestamp() + ACCESS_TOKEN_EXPIRE,
            },
            JWTData {
                name: self.name.clone(),
                id: self.id,
                exp: chrono::Utc::now().timestamp() + REFRESH_TOKEN_EXPIRE,
            },
        )
    }

    pub fn check(&self, name: &str, password: &str) -> bool {
        let salt = std::env::var("SERVER_PASSWORD_SALT").expect("SERVER_PASSWORD_SALT not found");
        let db_name = &self.name;
        let db_password = &self.password;
        let password = add_salt(password, &salt);
        if let Some(password) = password {
            return db_name == name && db_password == &hash(&password);
        }
        false
    }

    pub fn login(&self, name: &str, password: &str) -> Result<JWTToken> {
        if !self.check(name, password) {
            return Err(anyhow!("login failed"));
        }
        JWTToken::generate_token::<JWTData>(&self.to_login_response())
    }
}

impl JWTToken {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }

    pub fn generate_token<T>((access_claims, refresh_claims): &(T, T)) -> Result<Self>
    where
        T: Serialize + Sized,
    {
        let token =
            encode(&Header::default(), &access_claims, &jwt::KEYS.encoding).map_err(|e| {
                tracing::error!("generate_token error: {:?}", e);
                anyhow!(e)
            })?;
        let refresh_token = encode(
            &Header::default(),
            refresh_claims,
            &jwt::KEYS.refresh_encoding,
        )?;
        Ok(JWTToken::new(token, refresh_token))
    }
}
