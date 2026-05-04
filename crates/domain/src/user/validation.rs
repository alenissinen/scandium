use regex::Regex;
use std::sync::LazyLock;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^@\s]*?@[^@\s]*?\.[^@\s]*$").expect("Invalid email regex"));

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}
