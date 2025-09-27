mod error;
use crate::ais::asst;
use std::env;

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
    let oac = ais::new_oa_client()?; // Create OpenAI client wrapper

    let asst_id = asst::create_asst(
        &oac,
        asst::CreateConfig {
            name: "buddy-01".to_string(),
            model: "gpt-4o".to_string(),
        },
    )
    .await?;
    println!("Created assistant with ID: {}", asst_id);
    // println!("oac: {:?}", oac);
    Ok(())
}

//asst_cjIZjmFRw0mk4Nh0ORqttDBT
//asst_JGju8F3J3LQYZzKwO2BWOR5K
