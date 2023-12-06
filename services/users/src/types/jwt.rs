use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use common_types::{UserIdentifiers, UserRoles};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginTokenRespone {
    pub token: String,
    pub r#type: TokenType,
}

impl LoginTokenRespone {
    pub fn to_auth_header(&self) -> String {
        format!("{} {}", self.r#type, self.token)
    }
}

impl TryFrom<&str> for LoginTokenRespone {
    type Error = anyhow::Error;

    fn try_from(hdr: &str) -> Result<Self, Self::Error> {
        let mut hdr_parts = hdr.split(' ');
        let r#type = TokenType::from_str(
            hdr_parts
                .next()
                .ok_or(anyhow!("No type `Bearer` in JWT token."))?,
        )?;
        let token: String = hdr_parts
            .next()
            .ok_or(anyhow!("No token value in JWT token."))?
            .into();

        Ok(LoginTokenRespone { token, r#type })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum TokenType {
    Bearer,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bearer")
    }
}

impl FromStr for TokenType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bearer" => Ok(Self::Bearer),
            _ => Err(anyhow!("Unexpected JWT token type.")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String,
    pub id: Uuid,
    pub role: UserRoles,
    pub exp: i64,
}

pub fn generate_jwt_token(user: &UserIdentifiers, secret: &str) -> String {
    let claims = JWTClaims {
        sub: user.email.clone(),
        id: user.id,
        role: user.role,
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
    };

    let header = Header::default();
    let key = EncodingKey::from_secret(secret.as_ref());
    encode(&header, &claims, &key).expect("Failed to generate JWT token")
}

pub fn token_is_valid(
    token: &str,
    secret: &str,
) -> Result<TokenData<JWTClaims>, jsonwebtoken::errors::Error> {
    decode::<JWTClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
