use maturity_macro::maturity;

use crate::{Config, Error};

#[maturity(pending, issue = "#15")]
pub fn validate(config: &Config) -> Result<(), Error> {
    validate_name(&config.name)?;
    validate_age(config.age)?;

    Ok(())
}

fn validate_name(name: &str) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::MissingValue("name".into()));
    }

    Ok(())
}

fn validate_age(age: u8) -> Result<(), Error> {
    if age == 0 {
        return Err(Error::InvalidAge);
    }

    Ok(())
}
