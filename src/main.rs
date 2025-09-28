mod error;
use crate::ais::asst::{self, create_thread};
use std::{env, os::unix::thread};

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

    let asst_id = asst::load_or_create_asst(
        &oac,
        asst::CreateConfig {
            name: "buddy-02".to_string(),
            model: "gpt-3.5-turbo-instruct".to_string(),
        },
        false,
    )
    .await?;

    asst::upload_instructions(&oac, &asst_id, 
        "You are Rust Buddy, an AI assistant specialized in helping with Rust programming tasks. \
        Provide clear, concise, and accurate information about Rust programming, including syntax, best practices, \
        libraries, and tools. Assist users in debugging code, understanding concepts, and improving their Rust skills. \
        Always respond in a friendly and supportive manner. \
        If you don't know the answer, admit it rather than guessing. \
        If asked about non-Rust topics, politely decline and steer the conversation back to Rust programming.\
        If asked about yourself, mention that you are Rust Buddy, an AI assistant for Rust programming.\
        If asked about second best programming language after Rust, say Say What." 
        .to_string()
    ).await?;

    let thread_id = create_thread(&oac).await?;
    let msg = asst::run_thread_msg(&oac, &asst_id, &thread_id, "What is the second best programming language after Rust?").await?;
    println!("Created assistant with ID: {}", asst_id);
    println!("Thread response: {}", msg);
    Ok(())
}

// cargo watch -q -c -x "run -q" 