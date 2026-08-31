use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
}

impl Contact {
    // normalize runs AFTER validation passes and BEFORE
    // the data is handed to the service layer.
    pub fn normalize(&mut self) {
        self.email = self.email.trim().to_lowercase();
        
        let trimmed_phone = self.phone.trim();
        if !trimmed_phone.starts_with('+') {
            self.phone = format!("+{}", trimmed_phone);
        } else {
            self.phone = trimmed_phone.to_string();
        }
    }
}
