use regex::Regex;
use std::sync::LazyLock;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").expect("Invalid email regex"));

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_passes() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("test.name@domain.fi"));
        assert!(is_valid_email("user+tag@example.org"));
    }

    #[test]
    fn invalid_emails_fail() {
        assert!(!is_valid_email("notanemail"));
        assert!(!is_valid_email("@domain.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@domain"));
        assert!(!is_valid_email("user @example.com"));
    }

    #[test]
    fn empty_string_fails() {
        assert!(!is_valid_email(""));
    }
}
