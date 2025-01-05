pub fn mail(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return false;
    }

    let local_part = parts[0];
    let domain_part = parts[1];

    if local_part.is_empty() || domain_part.is_empty() {
        return false;
    }

    if !domain_part.contains('.') {
        return false;
    }

    let domain_chars: Vec<char> = domain_part.chars().collect();
    if !domain_chars.first().unwrap().is_alphanumeric()
        || !domain_chars.last().unwrap().is_alphanumeric()
    {
        return false;
    }

    true
}
