// 获取客户端账号和密码（密码有 sha256 混淆）
// 对比配置文件中的账号和密码
// 返回是否验证成功，并返回包含 name 和 id 和 token

use anyhow::{Result, anyhow};
use blake2::{Blake2b512, Digest};
use jsonwebtoken::{Header, encode, Validation, decode};
use serde::{Deserialize, Serialize};

mod jwt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Auth {
    pub name: String,
    pub id: u64,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginResponse {
    pub name: String,
    pub id: u64,
    pub token: String,
}

pub fn hash(msg: &str) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(msg.as_bytes());
    let res = hasher.finalize();
    hex::encode(res)
}

impl Auth {
    pub fn new(name: String, id: u64, password: String) -> Self {
        Self {
            name,
            id,
            password,
        }
    }

    pub fn check(&self, name: &str, password: &str) -> bool {
        let db_name = std::env::var("CLIENT_NAME").unwrap();
        let db_password = std::env::var("CLIENT_PASSWORD").unwrap();
        db_name == name && db_password == hash(password)
    }

    pub fn from_token(token: &str) -> Result<Self> {
        let token_data = decode::<Auth>(token, &jwt::KEYS.decoding, &Validation::default())
            .map_err(|e| {
                tracing::error!("validate_token error: {:?}", e);
                anyhow!(e)
            })?;
        Ok(token_data.claims)
    }

    pub fn generate_token(& self) -> Result<String> {
        let token = encode(&Header::default(), &self,  &jwt::KEYS.encoding)
        .map_err(|e| {
            tracing::error!("generate_token error: {:?}", e);
            anyhow!(e)
        })?;
        Ok(token)
    }


    pub fn login(&self, name: &str, password: &str) -> Result<LoginResponse> {
        if self.check(name, password) {
            let token = self.generate_token()?;
            Ok(LoginResponse {
                name: self.name.clone(),
                id: self.id,
                token,
            })
        } else {
            Err(anyhow!("login failed"))
        }
    }
}
