use showcase::{format, parse, validate};

fn main() {
    let text = r#"
    name = Alice
    age = 42
    country = UK"#;

    let config = parse(text).unwrap();

    validate(&config).unwrap();

    println!("{config:?}");
    println!();
    println!("{}", format(&config));
}
