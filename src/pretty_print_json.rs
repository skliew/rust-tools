extern crate serde_json;

use serde_json::{Value};

fn main() -> std::io::Result<()> {
    let value: Value = serde_json::from_reader(std::io::stdin())?;
    let pretty_string = serde_json::to_string_pretty(&value)?;
    println!("{}", pretty_string);
    Ok(())
}
