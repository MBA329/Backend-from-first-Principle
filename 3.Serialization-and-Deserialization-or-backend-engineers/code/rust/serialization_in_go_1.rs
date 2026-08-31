use serde::{Deserialize, Serialize};
use serde_json;
use chrono::{DateTime, Utc};

// ABSTRACTION: Serializer names a capability - "turn data into
// bytes and back" - without binding to a concrete format. Code
// depends on this contract, so JSON could be swapped for another
// format with zero changes to callers.
pub trait Serializer {
    fn serialize<T: Serialize>(&self, v: &T) -> Result<String, String>;
    fn deserialize<'a, T: Deserialize<'a>>(&self, data: &'a str) -> Result<T, String>;
}

// POLYMORPHISM: JSONSerializer implements Serializer. Any other
// format (YAML, Protobuf) could implement the same interface and
// be used interchangeably through a Serializer variable.
pub struct JSONSerializer;

impl Serializer for JSONSerializer {
    fn serialize<T: Serialize>(&self, v: &T) -> Result<String, String> {
        serde_json::to_string(v).map_err(|e| e.to_string()) // SERIALIZE: struct -> JSON string
    }
    
    fn deserialize<'a, T: Deserialize<'a>>(&self, data: &'a str) -> Result<T, String> {
        serde_json::from_str(data).map_err(|e| e.to_string()) // DESERIALIZE: JSON string -> struct
    }
}

// "INHERITANCE" via COMPOSITION: BaseModel holds shared fields;
// embedding it into User reuses them (and their json tags).
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseModel {
    #[serde(default)]
    pub id: i32,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: DateTime<Utc>,
}

// ENCAPSULATION: an exported struct whose JSON shape is controlled
// by macros. `password` uses `skip` -> private AND
// invisible to the JSON encoder, so it never leaks over the wire.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseModel,            // embedded -> inherits ID, CreatedAt
    pub name: String,
    
    #[serde(default = "default_active")]
    pub active: bool,
    
    pub address: Address,           // nested object
    
    #[serde(skip, default)]
    password: String,               // unexported: hidden from JSON output
}

fn default_active() -> bool { true }

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub country: String,
    pub phone: i32,
}

fn main() {
    let codec = JSONSerializer; // program to the interface

    let u = User {
        base: BaseModel { id: 1, created_at: Utc::now() },
        name: "Ada".to_string(),
        active: true,
        address: Address { country: "India".to_string(), phone: 123456 },
        password: "supersecret".to_string(),
    };

    // SERIALIZE - native Rust struct into the common JSON format
    let out = codec.serialize(&u).unwrap();
    println!("{}", out);
    // {"id":1,"created_at":"...","name":"Ada","active":true,
    //  "address":{"country":"India","phone":123456}}

    // DESERIALIZE - JSON received over HTTP back into a Rust struct
    let incoming = r#"{"name":"Lin","address":{"country":"IN","phone":42}}"#;
    let back: User = codec.deserialize(incoming).unwrap();
    println!("{} {}", back.name, back.address.country); // Lin IN
}
