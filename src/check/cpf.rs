pub fn cpf(cpf: &str) -> bool {
    let cpf: Vec<u8> = cpf
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    if cpf.len() != 11 || cpf.iter().all(|&d| d == cpf[0]) {
        return false;
    }

    let mut sum = 0;
    for i in 0..9 {
        sum += (cpf[i] as usize) * (10 - i);
    }
    let rest = (sum * 10) % 11;
    let first_digit = if rest == 10 { 0 } else { rest as u8 };

    sum = 0;
    for i in 0..10 {
        sum += (cpf[i] as usize) * (11 - i);
    }
    let rest = (sum * 10) % 11;
    let second_digit = if rest == 10 { 0 } else { rest as u8 };

    first_digit == cpf[9] && second_digit == cpf[10]
}
