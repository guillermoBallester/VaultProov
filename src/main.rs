mod error;

use std::env;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use axum::{routing::get, routing::post, Router};
use axum::extract::Multipart;
use c2pa::{Builder, Reader, Settings, Context, BuilderIntent};
use error::VaultProovError;

#[tokio::main]
async fn main() -> Result<(), VaultProovError> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(VaultProovError::Other("missing arguments".to_string()))
    }
    match args[1].as_str() {
        "verify" => {
            if args.len() != 3 {
                return Err(VaultProovError::Other("missing file name".to_string()))
            }
            let path = &args[2];
            let format = c2pa::format_from_path(path).ok_or(c2pa::Error::UnsupportedType)?;
            let mut file = File::open(path)?;
            let json_text = verify(&format, &mut file)?;
            println!("Successfully {}", json_text);
            Ok(())
        },
        "sign" => {
            if args.len() != 4 {
                return Err(VaultProovError::Other("missing file name".to_string()))
            }
            sign(&args[2], &args[3])?;
            Ok(())
        },
        "serve" => {
            let app = Router::new()
                .route("/health", get(health))
                .route("/verify", post(verify_handler));
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
            axum::serve(listener, app).await?;
            Ok(())
        }
        _ => Err(VaultProovError::Other("Invalid arguments".to_string()))
    }
}

fn verify<R: Read + Seek + Send>(format: &str, stream: &mut R) -> Result<String, VaultProovError> {
    let reader = Reader::from_stream(&format, stream)?;
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

async fn health() -> &'static str {
"OK"
}

async fn verify_handler(mut multipart: Multipart) -> Result<String, VaultProovError> {
   let file = multipart
       .next_field().await.map_err(|_| VaultProovError::Other("missing file".to_string()))?;
    let field = file.ok_or(VaultProovError::Other("missing file".to_string()))?;
    let content_type = field.content_type().ok_or(VaultProovError::Other("missing content type".to_string()))?.to_string();
    let bytes = field.bytes().await.map_err(|_| VaultProovError::Other("missing file".to_string()))?;
    verify(&content_type, &mut Cursor::new(&bytes))
}