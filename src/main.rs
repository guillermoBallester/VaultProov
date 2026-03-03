use std::env;
use std::process;
use c2pa::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "verify" {
        eprintln!("Usage: vaultproov verify <file>");
        process::exit(1);
    }

    let filename = &args[2];
    let reader = Reader::from_file(filename)?;
    println!("{}", reader.json());
    Ok(())
}
