mod error;

use std::env;
use c2pa::{Builder, Reader, Settings, Context, BuilderIntent};
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
            let json_text = verify(&args[2])?;
            println!("Successfully {}", json_text);
            Ok(())
        },
        "sign" => {
            if args.len() != 4 {
                return Err(VaultProovError::Other("missing file name".to_string()))
            }
            sign(&args[2], &args[3])?;
            Ok(())
        }
        _ => Err(VaultProovError::Other("Invalid arguments".to_string()))
    }
}

fn verify(filename: &str) -> Result<String, VaultProovError> {
    let reader = Reader::from_file(filename)?;
    Ok(reader.json())
}

fn sign(input: &str, output: &str) -> Result<(), VaultProovError> {
    let settings = std::fs::read_to_string("test_settings.toml")?;
    let shared_context = Context::new()
        .with_settings(Settings::new()
            .with_toml(&settings)?)?
        .into_shared();
    let mut builder = Builder::from_shared_context(&shared_context);
    builder.set_intent(BuilderIntent::Edit);
    builder.sign_file(shared_context.signer()?, input, output)?;
    Ok(())
}