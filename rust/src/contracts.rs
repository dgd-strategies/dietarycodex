use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct RawContract {
    range: [f64; 2],
    required: Vec<String>,
}

#[derive(Debug)]
pub struct Contract {
    pub range: [f64; 2],
    pub required: Vec<&'static str>,
}

static CONTRACTS_JSON: &str = include_str!("../../schema/contracts.json");

static CONTRACT_MAP: Lazy<HashMap<&'static str, Contract>> = Lazy::new(|| {
    let raw: HashMap<String, RawContract> =
        serde_json::from_str(CONTRACTS_JSON).expect("invalid contracts.json");
    let mut map = HashMap::new();
    for (name, rc) in raw {
        let required: Vec<&'static str> = rc
            .required
            .into_iter()
            .map(|s| Box::leak(s.into_boxed_str()) as &'static str)
            .collect();
        map.insert(
            Box::leak(name.into_boxed_str()) as &'static str,
            Contract {
                range: rc.range,
                required,
            },
        );
    }
    map
});

pub fn get_contract(name: &str) -> Option<&'static Contract> {
    CONTRACT_MAP.get(name)
}

pub fn required_fields(name: &str) -> &'static [&'static str] {
    get_contract(name)
        .map(|c| c.required.as_slice())
        .unwrap_or(&[])
}

pub fn range(name: &str) -> [f64; 2] {
    get_contract(name)
        .map(|c| c.range)
        .unwrap_or([f64::NAN, f64::NAN])
}
