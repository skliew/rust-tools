use std::result::Result;
use serde_yaml::Value;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let value : Value = serde_yaml::from_reader(std::io::stdin())?;
    serde_json::to_writer_pretty(std::io::stdout(), &value)?;
    Ok(())
}
