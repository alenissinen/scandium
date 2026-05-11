use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub is_active: bool,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

// User type returned from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            location: user.location,
            created_at: user.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password_hash: "secret_hash".to_string(),
            display_name: "Test User".to_string(),
            avatar_url: None,
            location: None,
            phone: None,
            is_verified: false,
            is_active: true,
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn user_into_public_user_maps_fields_correctly() {
        let user = create_user();
        let public: PublicUser = user.clone().into();

        assert_eq!(public.id, user.id);
        assert_eq!(public.username, user.username);
        assert_eq!(public.display_name, user.display_name);
        assert_eq!(public.avatar_url, user.avatar_url);
        assert_eq!(public.location, user.location);
    }

    #[test]
    fn public_user_excludes_sensitive_fields() {
        let user = create_user();
        let public: PublicUser = user.into();
        let public_json = serde_json::to_string(&public).unwrap();

        assert!(!public_json.contains("password_hash"));
        assert!(!public_json.contains("secret_hash"));
        assert!(!public_json.contains("email"));
        assert!(!public_json.contains("phone"));
    }
}
