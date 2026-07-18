use maturity_macro::maturity;

use crate::{
    Config, Error,
    consts::{DEFAULT_COUNTRY, DEFAULT_NAME},
};

#[maturity(experimental)]
pub fn parse(input: &str) -> Result<Config, Error> {
    let mut name = DEFAULT_NAME.to_string();
    let mut age = 0;
    let mut country = DEFAULT_COUNTRY.to_string();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let (key, value) = split_key_value(line)?;

        match key {
            "name" => name = parse_line(value),
            "age" => age = parse_age(value)?,
            "country" => country = parse_line(value),
            _ => {}
        }
    }

    Ok(Config::new(name, age, country))
}

#[maturity(developing, todo = "Support comments", refactor = "Separate lexer")]
fn parse_line(value: &str) -> String {
    value.trim().to_owned()
}

#[maturity(experimental, todo = "Support quoted Strings")]
fn split_key_value(line: &str) -> Result<(&str, &str), Error> {
    let Some((key, value)) = line.split_once('=') else {
        return Err(Error::InvalidLine);
    };

    Ok((key.trim(), value.trim()))
}

#[maturity(experimental, refactor = "Move validation into `validator.rs`")]
fn parse_age(value: &str) -> Result<u8, Error> {
    value.parse().map_err(|_| Error::InvalidAge)
}
