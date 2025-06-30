use std::sync::Arc;

use async_openai::{
    config::OpenAIConfig, types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest}, Client
};

use crate::entitiy::ChatEntry;

const MODEL_NAME: &str = "gpt-4.1-mini";

pub async fn single_question(promt: &str) -> anyhow::Result<String> {
    let oa_client: Arc<Client<OpenAIConfig>> = Client::new().into(); // requires OPENAI_API_KEY env. var.
    let chat_client = oa_client.chat();

	let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestUserMessageArgs::default().content(promt).build()?.into()
    ];

	let msg_req = CreateChatCompletionRequest {
		model: MODEL_NAME.to_string(),
		messages,
		..Default::default()
	};
	let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat_response
        .choices
        .into_iter()
        .next()
        .unwrap();

	let answer = first_choice.message.content.unwrap();

	Ok(answer)
}

pub async fn chat_question(history: &[ChatEntry], promt: &str) -> anyhow::Result<String> {
    let oa_client: Arc<Client<OpenAIConfig>> = Client::new().into(); // requires OPENAI_API_KEY env. var.
    let chat_client = oa_client.chat();

    let mut messages: Vec<ChatCompletionRequestMessage> = Vec::with_capacity(history.len() + 2);

    for e in history {
        let req_msg = match e {
            ChatEntry::UserProm(txt) =>
                ChatCompletionRequestUserMessageArgs::default()
                .content(txt.as_str())
                .build()?
                .into(),
            ChatEntry::AssistantTextResponse(txt) =>
                ChatCompletionRequestAssistantMessageArgs::default()
                .content(txt.as_str())
                .build()?
                .into()
        };
        messages.push(req_msg);
    }

    // TODO: if messages is empty, then add system prompt.

    messages.push(ChatCompletionRequestUserMessageArgs::default().content(promt).build()?.into());

	let msg_req = CreateChatCompletionRequest {
		model: MODEL_NAME.to_string(),
		messages,
		..Default::default()
	};
	let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat_response
        .choices
        .into_iter()
        .next()
        .unwrap();

	let answer = first_choice.message.content.unwrap();

	Ok(answer)
}
