mod c2pa;
mod jpeg;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "verify" {
        eprintln!("Usage: vaultproov verify <file>");
        process::exit(1);
    }

    let filename = &args[2];
    let bytes = match fs::read(filename) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("File not found: {}", error);
            process::exit(1)
        }
    };

    if jpeg::is_jpeg(&bytes) {
        let c2pa = jpeg::find_c2pa_signature(&bytes);
        if let Some((position, length)) = c2pa {
            let slice = &bytes[position + 4..position + 2 + length];
            c2pa::parse_manifest(slice);
        } else {
            eprintln!("C2PA signature not found");
        }
    } else {
        eprintln!("File is not a JPEG");
        process::exit(1);
    }
}
