use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "cross_field_rules", skip_on_field_errors = false))]
pub struct Signup {
    #[validate(length(min = 8))]
    pub password: String,
    pub password_confirmation: String,
    pub married: bool,
    pub partner: Option<String>,
}

fn cross_field_rules(s: &Signup) -> Result<(), ValidationError> {
    if s.password != s.password_confirmation {
        return Err(ValidationError::new("passwords don't match"));
    }
    if s.married && s.partner.is_none() {
        return Err(ValidationError::new("partner name is required when married is true"));
    }
    Ok(())
}
