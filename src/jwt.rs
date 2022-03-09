use clap::{ Parser, Subcommand, Args };
use std::io;
use biscuit::JWT;
use biscuit::jws::{ Secret };
use biscuit::jwa::{ SignatureAlgorithm };
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
    Decode(DecodeFile)
}

#[derive(Args)]
struct DecodeFile {
    file_name: Option<String>,
}

fn decode_file(file: Option<String>) -> Result<(), Error> {
    match file {
        None => {
            // Read from STDIN
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let input = buffer.trim();
            let token = JWT::<serde_json::Value, biscuit::Empty>::new_encoded(&input);
            let decoded = token.into_decoded(&Secret::None, SignatureAlgorithm::None)?.unwrap_decoded();
            println!("{}", serde_json::to_string(&decoded.0)?);
            println!("{}", serde_json::to_string(&decoded.1)?);
            Ok(())
        },
        Some(file_name) => {
            println!("File {}", file_name);
            Ok(())
        }
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode(file) => {
            decode_file(file.file_name)?;
        }
    }
    Ok(())
}
