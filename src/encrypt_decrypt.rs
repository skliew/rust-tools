extern crate hex;
extern crate aesni;
extern crate clap;
extern crate rand;

use std::env;
use std::str;
use rand::random;
use std::error::Error;
use std::fmt;
use aesni::block_cipher_trait::generic_array::GenericArray;
use aesni::stream_cipher::{
    SyncStreamCipher, NewStreamCipher
};
use clap::{ App, Arg, SubCommand };

#[derive(Debug, Clone)]
struct ArgumentError;

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArgumentError")
    }
}
impl Error for ArgumentError {}

fn encrypt(input: &str, key: String) -> Result<String, Box<Error>> {
    let mut input_data = String::from(input).into_bytes();
    let iv : [u8; 16] = random();

    let nonce = GenericArray::clone_from_slice(&iv);
    let key_array = GenericArray::clone_from_slice(key.as_bytes());

    let mut cipher = aesni::Aes256Ctr::new(&key_array, &nonce);
    cipher.apply_keystream(&mut input_data);

    let hex_output = hex::encode(input_data);
    let iv_hex = hex::encode(iv);

    let result = format!("{}|{}", hex_output, iv_hex);
    Ok(result)
}

fn decrypt(input: &str, key: String) -> Result<String, Box<Error>> {
    let input_string = String::from(input);
    let splits: Vec<&str> = input_string.split("|").collect();

    if splits.len() < 2 {
        return Err(ArgumentError.into());
    }
    let mut encrypted_data = hex::decode(splits[0])?;
    let iv = hex::decode(splits[1])?;
    let nonce = GenericArray::clone_from_slice(&iv);
    let key_array = GenericArray::clone_from_slice(key.as_bytes());

    let mut cipher = aesni::Aes256Ctr::new(&key_array, &nonce);

    cipher.apply_keystream(&mut encrypted_data);

    let output = String::from_utf8(encrypted_data)?;
    Ok(output)
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("encrypt_decrypt")
        .version("0.1")
        .arg(Arg::with_name("key")
             .short("k")
             .long("key")
             .help("Key to encrypt/decrypt")
             .takes_value(true))
        .subcommand(SubCommand::with_name("encrypt")
                    .arg(Arg::with_name("INPUT")
                         .required(true)
                         .index(1)))
        .subcommand(SubCommand::with_name("decrypt")
                    .arg(Arg::with_name("INPUT")
                         .required(true)
                         .index(1)));
    let matches = app.get_matches();

    let key_match = matches.value_of("key");
    let key = match key_match {
        Some(k) => String::from(k),
        None => {
            let default_key = env::var("AES_CTR_KEY");
            match default_key {
                Ok(key) => key,
                Err(_e) => {
                    println!("Key should be provided. Please run command with --help for more info.");
                    std::process::exit(1);
                }
            }
        }
    };

    let subcommand = matches.subcommand();
    let result = match subcommand {
        ("encrypt", Some(sub)) => {
            let input = sub.value_of("INPUT").unwrap();
            encrypt(input, key)
        },
        ("decrypt", Some(sub)) => {
            let input = sub.value_of("INPUT").unwrap();
            decrypt(input, key)
        },
        _ => Ok(String::from("No subcommand"))
    };
    match result {
        Ok(result) => { println!("{}", result); },
        Err(err) => { println!("Error: {}", err); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_encrypt_decrypt() -> Result<(), Box<dyn Error>> {
        let input = "input_string";
        let random_bytes : [u8; 16] = random();
        let key = hex::encode(random_bytes);
        let key_refcell: RefCell<String> = RefCell::new(key);
        let encrypted = encrypt(input, key_refcell.borrow().to_string())?;
        let decrypted = decrypt(&encrypted, key_refcell.borrow().to_string())?;
        assert_eq!(input, decrypted);
        Ok(())
    }
}
