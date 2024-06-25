use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LightspeedApiError {
    pub message: String,
    pub status_code: u16,
}

impl fmt::Display for LightspeedApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lightspeed API Error ({}): {}", self.status_code, self.message)
    }
}

impl Error for LightspeedApiError {}