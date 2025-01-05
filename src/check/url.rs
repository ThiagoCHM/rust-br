pub fn url(url: &str) -> bool {
    let url_parts: Vec<&str> = url.split("://").collect();

    if url_parts.len() != 2 {
        return false;
    }

    let scheme = url_parts[0];
    let rest = url_parts[1];

    if scheme != "http" && scheme != "https" {
        return false;
    }

    let domain_and_path: Vec<&str> = rest.split('/').collect();

    if domain_and_path.is_empty() || domain_and_path[0].is_empty() {
        return false;
    }

    let domain = domain_and_path[0];

    if !domain.contains('.') {
        return false;
    }

    let domain_parts: Vec<&str> = domain.split('.').collect();

    for part in domain_parts {
        if part.is_empty()
            || !part.chars().next().unwrap().is_alphanumeric()
            || !part.chars().last().unwrap().is_alphanumeric()
        {
            return false;
        }
    }

    true
}
