use std::env;
use std::process;

fn main() {
    println!("Starting VaultProov...");

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("No arguments provided");
            process::exit(1)
        }
        2 => {
            println!("Argument missing, format should be: verify file.xx");
            process::exit(1)
        }
        3 => {
            if args[1] == "verify" {
                println!("Verifying file: {}", args[2])
            } else {
                println!("Invalid argument: {}", args[1]);
                process::exit(1)
            }
        }
        _ => {
            println!("Too many arguments");
            process::exit(1)
        }
    }
}
