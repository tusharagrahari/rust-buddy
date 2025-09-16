mod error;

pub use self::error::{Error, Result};
mod ais;
mod buddy;



#[tokio::main]
async fn main() {
    match start().await {
        Ok(_) => println!("Rust Buddy started successfully."),
        Err(e) => eprintln!("Error starting Rust Buddy: {}", e),
    }
}


async fn start() -> Result<()> {
    println!("Starting Rust Buddy...");
    Ok(())
}
