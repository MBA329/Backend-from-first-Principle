use serde::Deserialize;

// *Option<bool> so we can tell "false" apart from "missing".
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)] // reject stray keys
pub struct TypePayload {
    pub string_field: String,
    pub number_field: f64,
    pub array_field: Vec<String>,
    pub bool_field: Option<bool>,
}

// serde_json enforces the BASE types: a string for
// number_field / array_field / bool_field fails to deserialize.
