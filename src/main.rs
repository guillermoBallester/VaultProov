mod error;

use std::env;
use c2pa::Reader;
use error::VaultProovError;

fn main() -> Result<(), VaultProovError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(VaultProovError::Other("missing arguments".to_string()))
    }
    match args[1].as_str() {
        "verify" => {
            if args.len() != 3 {
                return Err(VaultProovError::Other("missing file name".to_string()))
            }
            let filename = &args[2];
            let reader = Reader::from_file(filename)?;
            println!("{}", reader.json());
            Ok(())
        },
        _ => Err(VaultProovError::Other("Invalid arguments".to_string()))
    }
}
