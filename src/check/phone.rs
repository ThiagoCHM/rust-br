use std::collections::HashMap;
use std::sync::OnceLock;

fn ddd_map() -> &'static HashMap<&'static str, (&'static str, &'static str)> {
    static DDD_MAP: OnceLock<HashMap<&str, (&str, &str)>> = OnceLock::new();
    DDD_MAP.get_or_init(|| {
        let mut map = HashMap::new();

        // Centro-Oeste
        map.insert("61", ("Distrito Federal", "Centro-Oeste"));
        map.insert("62", ("Goiás", "Centro-Oeste"));
        map.insert("64", ("Goiás", "Centro-Oeste"));
        map.insert("65", ("Mato Grosso", "Centro-Oeste"));
        map.insert("66", ("Mato Grosso", "Centro-Oeste"));
        map.insert("67", ("Mato Grosso do Sul", "Centro-Oeste"));

        // Nordeste
        map.insert("82", ("Alagoas", "Nordeste"));
        map.insert("71", ("Bahia", "Nordeste"));
        map.insert("73", ("Bahia", "Nordeste"));
        map.insert("74", ("Bahia", "Nordeste"));
        map.insert("75", ("Bahia", "Nordeste"));
        map.insert("77", ("Bahia", "Nordeste"));
        map.insert("85", ("Ceará", "Nordeste"));
        map.insert("88", ("Ceará", "Nordeste"));
        map.insert("98", ("Maranhão", "Nordeste"));
        map.insert("99", ("Maranhão", "Nordeste"));
        map.insert("83", ("Paraíba", "Nordeste"));
        map.insert("81", ("Pernambuco", "Nordeste"));
        map.insert("87", ("Pernambuco", "Nordeste"));
        map.insert("86", ("Piauí", "Nordeste"));
        map.insert("89", ("Piauí", "Nordeste"));
        map.insert("84", ("Rio Grande do Norte", "Nordeste"));
        map.insert("79", ("Sergipe", "Nordeste"));

        // Norte
        map.insert("68", ("Acre", "Norte"));
        map.insert("96", ("Amapá", "Norte"));
        map.insert("92", ("Amazonas", "Norte"));
        map.insert("97", ("Amazonas", "Norte"));
        map.insert("91", ("Pará", "Norte"));
        map.insert("93", ("Pará", "Norte"));
        map.insert("94", ("Pará", "Norte"));
        map.insert("69", ("Rondônia", "Norte"));
        map.insert("95", ("Roraima", "Norte"));
        map.insert("63", ("Tocantins", "Norte"));

        // Sudeste
        map.insert("27", ("Espírito Santo", "Sudeste"));
        map.insert("28", ("Espírito Santo", "Sudeste"));
        map.insert("31", ("Minas Gerais", "Sudeste"));
        map.insert("32", ("Minas Gerais", "Sudeste"));
        map.insert("33", ("Minas Gerais", "Sudeste"));
        map.insert("34", ("Minas Gerais", "Sudeste"));
        map.insert("35", ("Minas Gerais", "Sudeste"));
        map.insert("37", ("Minas Gerais", "Sudeste"));
        map.insert("38", ("Minas Gerais", "Sudeste"));
        map.insert("21", ("Rio de Janeiro", "Sudeste"));
        map.insert("22", ("Rio de Janeiro", "Sudeste"));
        map.insert("24", ("Rio de Janeiro", "Sudeste"));
        map.insert("11", ("São Paulo", "Sudeste"));
        map.insert("12", ("São Paulo", "Sudeste"));
        map.insert("13", ("São Paulo", "Sudeste"));
        map.insert("14", ("São Paulo", "Sudeste"));
        map.insert("15", ("São Paulo", "Sudeste"));
        map.insert("16", ("São Paulo", "Sudeste"));
        map.insert("17", ("São Paulo", "Sudeste"));
        map.insert("18", ("São Paulo", "Sudeste"));
        map.insert("19", ("São Paulo", "Sudeste"));

        // Sul
        map.insert("41", ("Paraná", "Sul"));
        map.insert("42", ("Paraná", "Sul"));
        map.insert("43", ("Paraná", "Sul"));
        map.insert("44", ("Paraná", "Sul"));
        map.insert("45", ("Paraná", "Sul"));
        map.insert("46", ("Paraná", "Sul"));
        map.insert("51", ("Rio Grande do Sul", "Sul"));
        map.insert("53", ("Rio Grande do Sul", "Sul"));
        map.insert("54", ("Rio Grande do Sul", "Sul"));
        map.insert("55", ("Rio Grande do Sul", "Sul"));
        map.insert("47", ("Santa Catarina", "Sul"));
        map.insert("48", ("Santa Catarina", "Sul"));
        map.insert("49", ("Santa Catarina", "Sul"));

        map
    })
}

pub struct PhoneValidationResult {
    pub is_valid: bool,
    pub phone_type: Option<String>,
    pub region: Option<String>,
    pub state: Option<String>,
}

pub fn phone(phone: &str) -> PhoneValidationResult {
    let cleaned: String = phone.chars().filter(|c| c.is_digit(10)).collect();
    let length = cleaned.len();

    let is_valid_length = length == 10 || length == 11;

    if !is_valid_length {
        return PhoneValidationResult {
            is_valid: false,
            phone_type: None,
            region: None,
            state: None,
        };
    }

    let phone_type = if length == 10 {
        Some("Fixo".to_string())
    } else {
        Some("Móvel".to_string())
    };

    let (state, region) = if length == 10 || length == 11 {
        let ddd = &cleaned[0..2];
        ddd_map().get(ddd).map_or((None, None), |&(s, r)| {
            (Some(s.to_string()), Some(r.to_string()))
        })
    } else {
        (None, None)
    };

    let is_valid = is_valid_length && state.is_some();

    PhoneValidationResult {
        is_valid,
        phone_type,
        region,
        state,
    }
}
