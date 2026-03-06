mod error;

use std::env;
use c2pa::{Builder, Reader, Settings,Context, BuilderIntent};
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
        "sign" => {
            if args.len() != 4 {
                return Err(VaultProovError::Other("missing file name".to_string()))
            }
            let output_file = args[3].as_str();
            let settings = std::fs::read_to_string("test_settings.toml")?;
            let shared_context = Context::new()
                .with_settings(Settings::new()
                    .with_toml(&settings)?)?
                .into_shared();
            let mut builder = Builder::from_shared_context(&shared_context);
            builder.set_intent(BuilderIntent::Edit);
            builder.sign_file( shared_context.signer()?, &args[2], output_file)?;
            Ok(())
        }
        _ => Err(VaultProovError::Other("Invalid arguments".to_string()))
    }
}
