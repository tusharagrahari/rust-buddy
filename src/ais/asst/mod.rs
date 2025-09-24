use async_openai::types::{responses::{FileSearch, Function}, AssistantObject, AssistantTools, CreateAssistantRequest};
use derive_more::derive::{Deref, Display, From};

use crate::{ais::OaClient, Result};

const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];
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
        tools: Some(vec![
            AssistantTools::CodeInterpreter]),
        ..Default::default()
    })
    .await?;
    Ok(asst_obj.id.into())
}

pub async fn first_by_name(oac: &OaClient, name: &str) -> Result<Option<AssistantObject>> {
    // let query = (); // Use an empty tuple as the query if no specific query is needed
    let oa_assts = oac.assistants().list(DEFAULT_QUERY).await?;
    Ok(oa_assts.data.iter().find(|a| a.name.as_deref() == Some(name)).cloned())
}