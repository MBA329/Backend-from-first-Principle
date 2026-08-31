use serde::Deserialize;
use validator::{Validate, ValidationError};
use chrono::{NaiveDate, Utc};

#[derive(Debug, Deserialize, Validate)]
pub struct Profile {
    #[validate(custom = "validate_not_in_future")]
    pub date_of_birth: String,
    // gte/lte cover the "430 is impossible" semantic bound
    #[validate(range(min = 1, max = 120))]
    pub age: i32,
}

// Type & syntax can't express "not in the future" , 
// semantics need real logic against the real clock.
fn validate_not_in_future(date: &str) -> Result<(), ValidationError> {
    let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| ValidationError::new("invalid date"))?;
    if parsed > Utc::now().naive_utc().date() {
        return Err(ValidationError::new("date of birth cannot be in the future"));
    }
    Ok(())
}
