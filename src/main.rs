use std::env;
use std::fs;
use std::process;

fn is_jpeg(bytes: &[u8]) -> bool {
    bytes[0] == 0xFF && bytes[1] == 0xD8
        && bytes[bytes.len() - 1] == 0xD9 && bytes[bytes.len() - 2] == 0xFF
}

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

    if bytes.len() < 4 {
        eprintln!("File too small: {}", bytes.len());
        process::exit(1);
    }
    if is_jpeg(&bytes) {
        println!("File is a JPEG");
    } else {
        eprintln!("File is not a JPEG");
        process::exit(1);
    }
}
