pub fn format_tokens(tokens: i64) -> String {
    if tokens >= 1_000_000 {
        format!("{:.2}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        format!("{:.2}k", tokens as f64 / 1_000.0)
    } else {
        tokens.to_string()
    }
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.4}", amount)
}

pub fn sanitize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

pub fn validate_username(username: &str) -> Result<String, String> {
    let trimmed = username.trim();
    if trimmed.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    if trimmed.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    Ok(trimmed.to_string())
}