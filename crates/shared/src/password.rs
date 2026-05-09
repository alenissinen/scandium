use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())
        .map(|h| h.to_string())
}

#[cfg(test)]
mod tests {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    use super::*;

    #[test]
    fn hash_password_returns_ok() {
        assert!(hash_password("testpassword111").is_ok());
    }

    #[test]
    fn hash_password_produces_verifiable_hash() {
        let password = "testpassword111";
        let hash = hash_password(password).unwrap();
        let parsed_hash = PasswordHash::new(&hash).unwrap();

        assert!(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        );
    }

    #[test]
    fn different_passwords_produce_different_hashes() {
        let hash1 = hash_password("password123").unwrap();
        let hash2 = hash_password("password321").unwrap();

        assert_ne!(hash1, hash2);
    }
}
