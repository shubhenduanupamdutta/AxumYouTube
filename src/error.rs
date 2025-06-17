use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct MainError {
    message: String,
}

impl MainError {
    pub fn new(message: String) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
