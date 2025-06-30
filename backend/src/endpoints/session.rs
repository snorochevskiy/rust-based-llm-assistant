use axum::{body::Body, http::{header::{COOKIE, SET_COOKIE}, HeaderMap}, response::{IntoResponse, Response}};

use crate::session_mgmt::{self, SESSIONS};

pub async fn init_session(headers: HeaderMap) -> impl IntoResponse {
    if let Some(session_id) = extract_session_id_from_cookie_header(&headers) {
        let map = SESSIONS.lock().await;
        if map.contains_key(&session_id) {
            return Response::builder()
            .status(200)
            .body(Body::empty())
            .unwrap();
        }
    }

    let id = uuid::Uuid::new_v4().to_string();

    let mut lock = SESSIONS.lock().await;
    lock.insert(id.clone(), Vec::new());

    Response::builder()
        .status(200)
        .header(SET_COOKIE, format!("sessionid={id}"))
        .body(Body::empty())
        .unwrap()
}

pub fn extract_session_id_from_cookie_header(headers: &HeaderMap) -> Option<String> {
    if let Some(cookie_header) = headers.get(COOKIE) {
        return  session_mgmt::parse_seesion_id_from_cookie_map(cookie_header.to_str().unwrap());
    }
    None
}