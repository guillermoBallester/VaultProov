use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Starting VaultProov...");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "verify" {
        println!("Invalid number of arguments");
        process::exit(1);
    }

    let filename = &args[2];
    match fs::read(filename) {
        Ok(bytes) => {
            println!("File read successfully {}", bytes.len());
        }
        Err(error) => {
            println!("File not found: {}", error);
            process::exit(1)
        }
    }
}
