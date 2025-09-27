use async_openai::types::{
    AssistantObject, AssistantTools, CreateAssistantRequest,
    responses::{FileSearch, Function},
};
use derive_more::derive::{Deref, Display, From};

use crate::{Result, ais::OaClient};

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

// CRUD on asst ---------------------------
pub async fn create_asst(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {
    let oa_assts = oac.assistants();

    let asst_obj = oa_assts
        .create(CreateAssistantRequest {
            name: Some(config.name),
            model: config.model,
            tools: Some(vec![AssistantTools::CodeInterpreter]),
            ..Default::default()
        })
        .await?;
    Ok(asst_obj.id.into())
}

pub async fn load_or_create_asst(
    oac: &OaClient,
    config: CreateConfig,
    recreate: bool, // if true, always create new, else load existing or create new if not found
) -> Result<AsstId> {
    let asst_obj = first_by_name(oac, &config.name).await?;
    let mut asst_id = asst_obj.map(|o| AsstId::from(o.id));

    // Delete asst if recreate true and asst_id
    if let(true, Some(id)) = (recreate, &asst_id) {
        delete(oac, id).await?;
        asst_id.take();
        println!("Assistance {} deleted", config.name);
    }
    
    // if asst_id exist
    if let Some(id) = asst_id {
        println!("Assistant {} loaded", config.name);
        Ok(id)
    } else { // creating new asst since asst_id does not exist
        let asst_name = config.name.clone();
        let asst_id = create_asst(oac, config).await?;
        Ok(asst_id)
    }
}

pub async fn delete(oac: &OaClient, asst_id: &AsstId) -> Result<()> {
    let oa_assts = oac.assistants();
    oa_assts.delete(&asst_id).await?;
    Ok(())
}

pub async fn first_by_name(oac: &OaClient, name: &str) -> Result<Option<AssistantObject>> {
    //Wrapped in result as it involves async call that can fail and in Option as the assistant with given name may not exist
    // let query = (); // Use an empty tuple as the query if no specific query is needed
    let oa_assts = oac.assistants().list(DEFAULT_QUERY).await?;
    Ok(oa_assts
        .data
        .iter()
        .find(|a| a.name.as_deref() == Some(name))
        .cloned())
}