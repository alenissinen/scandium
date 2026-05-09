use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn create_access_token(
        &self,
        user_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::minutes(60)).timestamp(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn create_refresh_token(
        &self,
        user_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::days(14)).timestamp(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(token, &self.decoding_key, &Validation::default())?;
        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_service() -> JwtService {
        JwtService::new("test_secret_key")
    }

    #[test]
    fn create_access_token_returns_token() {
        let service = create_service();
        let result = service.create_access_token(Uuid::new_v4());

        assert!(result.is_ok());
    }

    #[test]
    fn create_refresh_token_returns_token() {
        let service = create_service();
        let result = service.create_refresh_token(Uuid::new_v4());

        assert!(result.is_ok());
    }

    #[test]
    fn verify_valid_token_returns_claims() {
        let service = create_service();
        let token = service.create_access_token(Uuid::new_v4()).unwrap();
        let claims = service.verify(&token);

        assert!(claims.is_ok());
    }

    #[test]
    fn token_subject_matches_user_id() {
        let service = create_service();
        let user_id = Uuid::new_v4();
        let token = service.create_access_token(user_id).unwrap();
        let claims = service.verify(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    fn verify_invalid_token_fails() {
        let service = create_service();
        let result = service.verify("not.a.valid.token");

        assert!(result.is_err());
    }

    #[test]
    fn verify_wrong_secret_fails() {
        let service1 = create_service();
        let service2 = JwtService::new("different_secret");
        let token = service1.create_access_token(Uuid::new_v4()).unwrap();
        let result = service2.verify(&token);

        assert!(result.is_err());
    }
}
