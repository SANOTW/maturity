use maturity_macro::maturity;

#[derive(Debug, Clone)]
#[maturity(stable)]
pub struct Config {
    pub name: String,
    pub age: u8,
    pub country: String,
}

impl Config {
    #[maturity(stable)]
    pub fn new(name: String, age: u8, country: String) -> Self {
        Self { name, age, country }
    }
}
