use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
    pub chat_id: i64,
    pub thread_id: Option<i32>,
}
