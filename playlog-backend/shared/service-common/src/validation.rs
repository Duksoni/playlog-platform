fn escape_like(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch == '\\' || ch == '%' || ch == '_' {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

pub fn like_pattern(input: &str) -> String {
    format!("%{}%", escape_like(input))
}

pub fn sanitize_ext(ext: &str) -> String {
    let cleaned: String = ext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(5)
        .collect();
    if cleaned.is_empty() {
        String::from("bin")
    } else {
        cleaned.to_lowercase()
    }
}

pub fn is_allowed_mime(field: &str, mime: &str) -> bool {
    match field {
        "cover" | "screenshot" => matches!(mime, "image/jpeg" | "image/png" | "image/webp"),
        "trailer" => matches!(mime, "video/mp4" | "video/webm"),
        _ => false,
    }
}
