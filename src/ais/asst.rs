use std::time::Duration;

use async_openai::types::{
    responses::{FileSearch, Function}, AssistantObject, AssistantTools, AssistantToolsFileSearch, AssistantToolsFunction, CreateAssistantRequest, CreateRunRequest, CreateThreadRequest, MessageRole, ModifyAssistantRequest, RunStatus, ThreadObject
};
use console::Term;
use derive_more::derive::{Deref, Display, From};
use tokio::time::sleep;

use crate::{ais::{msg::{get_text_content, user_msg}, OaClient}, Result};

const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];
const POLLING_DURATION_MS: u64 = 500; // 500 milliseconds
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
            tools: None,
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
    if let (true, Some(id)) = (recreate, &asst_id) {
        delete(oac, id).await?;
        asst_id.take();
        println!("Assistance {} deleted", config.name);
    }

    // if asst_id exist
    if let Some(id) = asst_id {
        println!("Assistant {} loaded", config.name);
        Ok(id)
    } else {
        // creating new asst since asst_id does not exist
        let asst_name = config.name.clone();
        let asst_id = create_asst(oac, config).await?;
        Ok(asst_id)
    }
}

pub async fn upload_instructions(
    oac: &OaClient,
    asst_id: &AsstId,
    inst_content: String,
) -> Result<()> {
    let oa_assts = oac.assistants();
    let modif = ModifyAssistantRequest {
        instructions: Some(inst_content),
        ..Default::default()
    };
    oa_assts.update(asst_id, modif).await?;
    Ok(())
}

pub async fn delete(oac: &OaClient, asst_id: &AsstId) -> Result<()> {
    // also need to delete the files associated with the asst
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

// CRUD on threads ---------------------------
pub async fn create_thread(oac: &OaClient) -> Result<ThreadId> {
    let oa_threads = oac.threads();
    let thread_obj = oa_threads
        .create(CreateThreadRequest {
            ..Default::default()
        })
        .await?;
    Ok(thread_obj.id.into())
}

pub async fn get_thread(oac: &OaClient, thread_id: &ThreadId) -> Result<ThreadObject> {
    let oa_threads = oac.threads();
    let thread_obj = oa_threads.retrieve(thread_id).await?;
    println!("Thread: {:?}", thread_obj);
    Ok(thread_obj)
}

pub async fn run_thread_msg(oac: &OaClient, asst_id: &AsstId,  thread_id: &ThreadId, msg: &str) -> Result<String> {
    let msg = user_msg(msg);

    // Attach message to thread
    let message_obj = oac
        .threads()
        .messages(thread_id)
        .create(msg)
        .await?;

    // Create a run for the thread
    let run_request = CreateRunRequest {
        assistant_id: asst_id.to_string(),
        ..Default::default()
    };
    let mut run = oac.threads().runs(thread_id).create(run_request).await?;

    // Loop to get results
    let term = Term::stdout();
    loop {
        term.write_str(">")?;
        let run = oac.threads().runs(thread_id).retrieve(&run.id).await?;
        term.write_str(">")?;

        match run.status {
            RunStatus::Completed => {
                term.write_str("Run completed\n")?;
                // Get the messages from the thread
                let response = get_first_thread_msg_content(oac, thread_id).await?;
                return Ok(response);
            }
            RunStatus::Queued | RunStatus::Incomplete | RunStatus::InProgress => {
                sleep(Duration::from_millis(POLLING_DURATION_MS)).await;
            }
            RunStatus::Failed | RunStatus::Expired | RunStatus::Cancelled => {
                return Err(format!("Run ended with status: {:?}", run.status).into());
            }
            _ => {
                if let Some(err) = run.last_error {
                    term.write_line(&format!("Run failed: {:?}", err))?;
                }
                return Err(format!("Run ended with status: {:?}", run.status).into());
            }
        }
        sleep(Duration::from_millis(POLLING_DURATION_MS)).await;
    }

}


pub async fn get_first_thread_msg_content(oac: &OaClient, thread_id: &ThreadId) -> Result<String> {
    static QUERY: [(&str, &str); 1] = [("limit", "20")]; // Limit to 20 message
    let oa_threads = oac.threads();
    let msgs = oa_threads.messages(thread_id).list(&QUERY).await?;
    // let first_msg = msgs
    //     .data
    //     .into_iter()
    //     .next()
    //     .ok_or_else(|| "No messages in thread")?;

    // let text = get_text_content(first_msg)?;
    // Ok(text)

    // let oa_threads = oac.threads();

    // Fetch all messages (no limit)
    // let msgs = oa_threads.messages(thread_id).list(Str::&[]).await?;

    // Find the latest assistant message
    let last_asst_msg = msgs
        .data
        .into_iter()
        .rev() // reverse so we start from newest
        .find(|m| m.role == MessageRole::Assistant)
        .ok_or("No assistant message found")?;

    let text = get_text_content(last_asst_msg)?;
    Ok(text)
}
