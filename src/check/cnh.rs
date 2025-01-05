pub fn cnh(cnh: &str) -> bool {
    let cnh_digits: Vec<u8> = cnh
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    if cnh_digits.len() != 11 {
        return false;
    }

    let mut sum = 0;
    for i in 0..9 {
        sum += (cnh_digits[i] as usize) * (9 - i);
    }
    let first_check_digit = (sum % 11) as u8;

    let first_check_digit = if first_check_digit >= 10 {
        0
    } else {
        first_check_digit
    };

    sum = 0;
    for i in 0..9 {
        sum += (cnh_digits[i] as usize) * (1 + i);
    }
    let second_check_digit = (sum % 11) as u8;

    let second_check_digit = if second_check_digit >= 10 {
        0
    } else {
        second_check_digit
    };

    first_check_digit == cnh_digits[9] && second_check_digit == cnh_digits[10]
}
