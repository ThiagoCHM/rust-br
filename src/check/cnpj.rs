pub fn cnpj(cnpj: &str) -> bool {
    let cnpj: Vec<u8> = cnpj
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    if cnpj.len() != 14 || cnpj.iter().all(|&d| d == cnpj[0]) {
        return false;
    }

    let mut sum = 0;
    let first_digit_weights = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    for i in 0..12 {
        sum += (cnpj[i] as usize) * first_digit_weights[i];
    }
    let rest = sum % 11;
    let first_digit = if rest < 2 { 0 } else { 11 - rest as u8 };

    if first_digit != cnpj[12] {
        return false;
    }

    sum = 0;
    let second_digit_weights = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    for i in 0..13 {
        sum += (cnpj[i] as usize) * second_digit_weights[i];
    }
    let rest = sum % 11;
    let second_digit = if rest < 2 { 0 } else { 11 - rest as u8 };

    second_digit == cnpj[13]
}
