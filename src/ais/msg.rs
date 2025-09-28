use crate::Result;
use async_openai::types::{
    CreateMessageRequest, CreateMessageRequestContent, MessageContent, MessageObject, MessageRole
};

pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
    CreateMessageRequest {
        role: MessageRole::User,
        content: CreateMessageRequestContent::Content(content.into()),
        ..Default::default()
    }
}

pub fn get_text_content(msg: MessageObject) -> Result<String> {
    // Extract the text content from the message object
    let msg_content = msg
        .content
        .into_iter()
        .next()
        .ok_or_else(|| "No content in message")?; // ok_or_else to provide custom error message lazily, this avoids allocating or computing the error unless the Option is None.

    let txt = match msg_content {
        MessageContent::Text(t) => t.text.value,
        _ => return Err("Message content is not text".into()),
    };

    Ok(txt)

}
