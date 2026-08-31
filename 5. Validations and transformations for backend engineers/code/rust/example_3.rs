use serde::Deserialize;
use validator::Validate;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PHONE_RE: Regex = Regex::new(r"^\+?[0-9]{7,15}$").unwrap();
}

#[derive(Debug, Deserialize, Validate)]
pub struct Contact {
    #[validate(email)]
    pub email: String, // local @ domain.tld
    #[validate(regex(path = "PHONE_RE", message = "invalid phone number format"))]
    pub phone: String, // checked below
    // custom validation for YYYY-MM-DD
    pub date: String, 
}
