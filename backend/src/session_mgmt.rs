use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};
use tokio::sync::Mutex;

use crate::entitiy::ChatEntry;

// TODO-XXX: move to middleware?
pub static SESSIONS: LazyLock<Mutex<HashMap<String, Vec<ChatEntry>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub const SESSIONID: &str = "sessionid";

pub fn parse_seesion_id_from_cookie_map(cookie: &str) -> Option<String> {
    let re = Regex::new(r"^(\w+)=(.+)$").unwrap();

    let pairs = cookie.split("&");
    for pair in pairs {
        for (_, [key, value]) in re.captures_iter(pair).map(|c| c.extract()) {
            if key == SESSIONID {
                return Some(value.to_string())
            }
        }
    }
    None
}

#[test]
fn test_parse_seesion_id_from_cookie_map() {
    let cookie = "sessionid=c8e1395b-368a-4e70-8bcf-2de5f0c3a74b";
    let sessionid = parse_seesion_id_from_cookie_map(cookie);
    println!("Session ID: {sessionid:?}");
}