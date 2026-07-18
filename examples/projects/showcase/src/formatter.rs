use maturity_macro::maturity;

use crate::Config;

#[maturity(stable)]
pub fn format(config: &Config) -> String {
    [
        format_line("name", &config.name),
        format_line("age", &config.age.to_string()),
        format_line("country", &config.country),
    ]
    .join("\n")
}

fn format_line(key: &str, value: &str) -> String {
    format!("{key} = {value}")
}
