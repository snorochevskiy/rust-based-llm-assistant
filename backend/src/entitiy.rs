use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEntry {
    UserProm(String),
    AssistantTextResponse(String),
}
