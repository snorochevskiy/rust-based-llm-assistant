use axum::{http::{HeaderMap, StatusCode}, response::IntoResponse, Json};

use crate::{endpoints::session::extract_session_id_from_cookie_header, entitiy::ChatEntry, llm, session_mgmt::SESSIONS};

pub async fn fetch_history(headers: HeaderMap) -> Json<serde_json::Value> {
    if let Some(session_id) = extract_session_id_from_cookie_header(&headers) {
        let lock = SESSIONS.lock().await;
        if let Some(vals) = lock.get(&session_id) {
            return Json(serde_json::json!(vals.clone()));
        }
    }
    Json(serde_json::json!(vec!["".to_string()]))
}

pub async fn new_chat(headers: HeaderMap) -> StatusCode {
    if let Some(session_id) = extract_session_id_from_cookie_header(&headers) {
        let mut lock = SESSIONS.lock().await;
        if let Some(v) = lock.get_mut(&session_id) {
            v.clear();
        }
    }
    StatusCode::OK
}

pub async fn single_question(headers: HeaderMap, promt: String) -> Json<serde_json::Value> {
    let response_msg = llm::single_question(&promt).await.unwrap();

    if let Some(session_id) = extract_session_id_from_cookie_header(&headers) {
        let mut lock = SESSIONS.lock().await;
        if let Some(vals) = lock.get_mut(&session_id) {
            vals.push(ChatEntry::UserProm(promt));
            vals.push(ChatEntry::AssistantTextResponse(response_msg.clone()));
        }
    }

    Json(serde_json::json!(vec![ChatEntry::AssistantTextResponse(response_msg.clone())]))
}

pub async fn chat_question(headers: HeaderMap, promt: String) -> impl IntoResponse {
    let Some(session_id) = extract_session_id_from_cookie_header(&headers) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let chat_history = {
        let lock = SESSIONS.lock().await;
        lock.get(&session_id).map(|v|v.to_owned()).unwrap_or_default()
    };


    let response_msg = llm::chat_question(&chat_history, &promt).await.unwrap();

    if let Some(session_id) = extract_session_id_from_cookie_header(&headers) {
        let mut lock = SESSIONS.lock().await;
        if let Some(vals) = lock.get_mut(&session_id) {
            vals.push(ChatEntry::UserProm(promt));
            vals.push(ChatEntry::AssistantTextResponse(response_msg.clone()));
        }
    }

    Json(vec![ChatEntry::AssistantTextResponse(response_msg.clone())])
        .into_response()
    
}