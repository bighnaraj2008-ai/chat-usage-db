use chat_usage_db::utils::{format_tokens, format_currency, sanitize_email, validate_username};

#[test]
fn test_format_tokens() {
    assert_eq!(format_tokens(500), "500");
    assert_eq!(format_tokens(1500), "1.50k");
    assert_eq!(format_tokens(2_500_000), "2.50M");
}

#[test]
fn test_format_currency() {
    assert_eq!(format_currency(0.0025), "$0.0025");
    assert_eq!(format_currency(12.3456), "$12.3456");
}

#[test]
fn test_sanitize_email() {
    assert_eq!(sanitize_email("  Test@Example.COM  "), "test@example.com");
}

#[test]
fn test_validate_username() {
    assert!(validate_username("valid_user").is_ok());
    assert!(validate_username("").is_err());
    assert!(validate_username("ab").is_err());
}