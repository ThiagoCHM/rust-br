pub fn passport(passport: &str) -> bool {
    if passport.len() != 8 {
        return false;
    }

    let (prefix, suffix) = passport.split_at(2);

    if !prefix.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    if !suffix.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    true
}
