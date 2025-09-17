use async_openai::types::{AssistantTools, CreateAssistantRequest};
use derive_more::derive::{Deref, Display, From};

use crate::{ais::OaClient, Result};

pub struct CreateConfig {
    pub name: String,
    pub model: String,
}
#[derive(Debug, From, Deref, Display)]
pub struct AsstId(pub String);

#[derive(Debug, From, Deref, Display)]
pub struct ThreadId(pub String);

#[derive(Debug, From, Deref, Display)]
pub struct FileId(pub String);



pub async fn create_asst(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {
    let oa_assts = oac.assistants();

    let asst_obj = oa_assts
    .create(CreateAssistantRequest {
        name: Some(config.name),
        model: config.model,
        ..Default::default()
    })
    .await?;
    Ok(asst_obj.id.into())
}