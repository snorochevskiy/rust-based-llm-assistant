use leptos::prelude::Error;

use crate::entity::ChatEntry;

pub async fn init_session() -> Result<String, Error> {
    let res = reqwasm::http::Request::get("http://localhost:8080/api/init-session")
        .credentials(reqwasm::http::RequestCredentials::Include)
        .send().await?
        .text().await?;
    Ok(res)
}

pub async fn load_history() -> Result<Vec<ChatEntry>, Error> {
    let res = reqwasm::http::Request::get("http://localhost:8080/api/chat/load-history")
        .credentials(reqwasm::http::RequestCredentials::Include)
        .send().await?
        .json::<Vec<ChatEntry>>().await?;
    Ok(res)
}

pub async fn new_chat() -> Result<(), Error> {
    reqwasm::http::Request::get("http://localhost:8080/api/chat/new")
        .credentials(reqwasm::http::RequestCredentials::Include)
        .send().await?;
    Ok(())
}


pub async fn send_promt(promt: &str) -> Result<Vec<ChatEntry>, Error> {
    let res = reqwasm::http::Request::post("http://localhost:8080/api/llm/chat-question")
        .body(promt)
        .credentials(reqwasm::http::RequestCredentials::Include)
        .send().await?
        .json::<Vec<ChatEntry>>().await?;
    Ok(res)
}