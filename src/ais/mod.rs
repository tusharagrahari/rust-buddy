pub mod asst;
use async_openai::{Client, config::OpenAIConfig};
use std::env;
pub mod msg;

use crate::Result;

const ENV_OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub type OaClient = Client<OpenAIConfig>; //type alias for async_openai Client with OpenAIConfig

pub fn new_oa_client() -> Result<OaClient> {
    dotenvy::dotenv().ok();
    if env::var(ENV_OPENAI_API_KEY).is_ok() {
        Ok(Client::new())
    } else {
        println!("Environment variable {ENV_OPENAI_API_KEY} not present");
        Err("Environment variable OPENAI_API_KEY not present".into())
    }
}
