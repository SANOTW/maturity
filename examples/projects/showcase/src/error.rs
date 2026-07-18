use maturity_macro::maturity;

#[derive(Debug)]
#[maturity(stable)]
pub enum Error {
    InvalidLine,
    MissingValue(String),
    InvalidAge,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLine => write!(f, "Invalid configuration line"),
            Error::MissingValue(key) => write!(f, "Missing value for '{key}'"),
            Error::InvalidAge => write!(f, "Age must be a valid number"),
        }
    }
}

impl std::error::Error for Error {}
