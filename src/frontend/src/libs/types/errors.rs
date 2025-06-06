use std::fmt;

#[derive(Debug, Default)]
pub struct ErrorStr {
    error: String,
}

impl fmt::Display for ErrorStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl ErrorStr {
    pub fn new<S>(error: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            error: error.into(),
        }
    }
}
