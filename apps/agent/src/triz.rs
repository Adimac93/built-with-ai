//! TRIZ reference data: 39 engineering parameters, 40 inventive principles,
//! and the 39×39 Altshuller contradiction matrix. Ported verbatim from the
//! Python `app/triz.py`; data embedded from `triz.json`.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct TrizData {
    parameters: HashMap<String, String>,
    principles: HashMap<String, String>,
    /// `matrix[improving][worsening] = [principle, ...]`; diagonal absent,
    /// missing cell = no recommendation.
    matrix: HashMap<String, HashMap<String, Vec<u8>>>,
}

fn data() -> &'static TrizData {
    static DATA: OnceLock<TrizData> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../triz.json")).expect("triz.json is embedded and valid")
    })
}

/// Looks up recommended principles for an (improving, worsening) parameter
/// pair. Errors when either parameter is outside 1-39 (Python raised
/// `ValueError`); returns an empty list for the diagonal or a missing cell.
pub fn lookup_principles(improving: i64, worsening: i64) -> Result<Vec<u8>, String> {
    if !(1..=39).contains(&improving) || !(1..=39).contains(&worsening) {
        return Err(format!(
            "TRIZ parameters must be 1-39, got improving={improving}, worsening={worsening}"
        ));
    }
    if improving == worsening {
        return Ok(Vec::new());
    }
    Ok(data()
        .matrix
        .get(&improving.to_string())
        .and_then(|row| row.get(&worsening.to_string()))
        .cloned()
        .unwrap_or_default())
}

/// Parameter name with the Python fallback `"Parameter {n}"`.
pub fn get_parameter_name(n: i64) -> String {
    data()
        .parameters
        .get(&n.to_string())
        .cloned()
        .unwrap_or_else(|| format!("Parameter {n}"))
}

/// Principle name with the Python fallback `"Principle {n}"`.
pub fn get_principle_name(n: u8) -> String {
    data()
        .principles
        .get(&n.to_string())
        .cloned()
        .unwrap_or_else(|| format!("Principle {n}"))
}

/// `{ "number": "<n>", "name": ... }` — `number` is a stringified int; the
/// frontend joins these strings directly.
pub fn get_principle_description(n: u8) -> serde_json::Value {
    json!({ "number": n.to_string(), "name": get_principle_name(n) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_39_parameters_and_40_principles() {
        assert_eq!(data().parameters.len(), 39);
        assert_eq!(data().principles.len(), 40);
    }

    #[test]
    fn all_matrix_cells_contain_valid_principles() {
        for (improving, row) in &data().matrix {
            for (worsening, principles) in row {
                for p in principles {
                    assert!(
                        (1..=40).contains(p),
                        "matrix[{improving}][{worsening}] has invalid principle {p}"
                    );
                }
            }
        }
    }

    #[test]
    fn diagonal_returns_empty() {
        for n in 1..=39 {
            assert_eq!(lookup_principles(n, n).unwrap(), Vec::<u8>::new());
        }
    }

    #[test]
    fn out_of_range_parameters_error() {
        assert!(lookup_principles(0, 5).is_err());
        assert!(lookup_principles(5, 40).is_err());
        assert!(lookup_principles(-1, 100).is_err());
    }

    #[test]
    fn known_cells_have_recommendations() {
        // Reliability (27) improving vs Volume of moving object (7) worsening.
        assert!(!lookup_principles(27, 7).unwrap().is_empty());
        // Reliability (27) vs Loss of substance (23) — present in the matrix.
        assert!(!lookup_principles(27, 23).unwrap().is_empty());
    }

    #[test]
    fn name_lookups_and_fallbacks() {
        assert_eq!(get_parameter_name(27), "Reliability");
        assert_eq!(get_parameter_name(0), "Parameter 0");
        assert_eq!(get_principle_name(35), "Parameter changes");
        assert_eq!(get_principle_name(41), "Principle 41");
    }

    #[test]
    fn principle_description_has_string_number() {
        let desc = get_principle_description(35);
        assert_eq!(desc["number"], "35");
        assert!(desc["name"].is_string());
    }
}
