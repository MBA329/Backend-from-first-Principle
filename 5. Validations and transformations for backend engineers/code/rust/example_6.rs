use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Pagination {
    // Serde automatically CASTS (transform) string from query param 
    // into int before we can VALIDATE the numbers.
    #[validate(range(min = 1, max = 499))]
    pub page: i32,
    #[validate(range(min = 1, max = 9999))]
    pub limit: i32,
}
