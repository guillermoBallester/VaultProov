mod jpeg;

use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Starting VaultProov...");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "verify" {
        eprintln!("Usage: vaultproov verify <file>");
        process::exit(1);
    }

    let filename = &args[2];
    let bytes =  match fs::read(filename) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("File not found: {}", error);
            process::exit(1)
        }
    };

    if jpeg::is_jpeg(&bytes) {
        println!("File is a JPEG");
    } else {
        eprintln!("File is not a JPEG");
        process::exit(1);
    }
}
