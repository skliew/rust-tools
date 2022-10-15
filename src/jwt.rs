use clap::{ Parser, Subcommand, Args };
use std::io;
use std::fs;
use biscuit::JWT;
use biscuit::jws::Secret;
use biscuit::jwa::SignatureAlgorithm;
use biscuit::errors::Error;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Decode(DecodeOptions),
    Encode(EncodeOptions)
}

#[derive(Args)]
struct DecodeOptions {
    file_name: Option<String>,
}

#[derive(Args)]
struct EncodeOptions {
    #[clap(short, long)]
    template: String,
}

fn decode_file(file: Option<String>) -> Result<(), Error> {
    let mut buffer;
    match file {
        None => {
            buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
        },
        Some(file_name) => {
            buffer = fs::read_to_string(file_name)?;
        }
    }
    let input = buffer.trim();
    let token = JWT::<serde_json::Value, biscuit::Empty>::new_encoded(&input);
    let decoded = token.into_decoded(&Secret::None, SignatureAlgorithm::None)?.unwrap_decoded();
    println!("Header:");
    println!("-------");
    println!("{}", serde_json::to_string_pretty(&decoded.0)?);
    println!("\nPayload:");
    println!("--------");
    println!("{}", serde_json::to_string_pretty(&decoded.1)?);
    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode(options) => {
            decode_file(options.file_name)?;
        }
        Commands::Encode(options) => {
            // decode_file(options.template)?;
        }
    }
    Ok(())
}
