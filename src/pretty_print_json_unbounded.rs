use serde_json::{ Value, Deserializer };
use serde_json::de::IoRead;
use serde::de;
use std::io::Read;
use std::error::Error;

fn from_reader<R, T>(read : R) -> Result<T, Box<dyn Error>>
where
R : Read,
T : de::DeserializeOwned
{
    let de_input = IoRead::new(read);
    let mut de = Deserializer::new(de_input);
    de.disable_recursion_limit();
    let value = de::Deserialize::deserialize(&mut de)?;

    de.end()?;
    Ok(value)
}

fn main() -> Result<(), Box<dyn Error>> {
    let value: Value = from_reader(std::io::stdin())?;
    let pretty_string = serde_json::to_string_pretty(&value)?;
    println!("{}", pretty_string);
    Ok(())
}
