use std::env;

fn main() {
    println!("Starting VaultProov...");

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("No arguments provided"),
        2 => println!("Argument missing, format should be: --verify file.xx"),
        3 => println!("arguments {}, {}", args[1], args[2]),
        _ => println!("Too many arguments"),
    }
}
