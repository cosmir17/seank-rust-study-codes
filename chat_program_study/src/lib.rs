use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)] // partialEq that is used to define partial equality between two values of the same type
//it's often used to implement comparison operators such as ==, != signs
pub enum Client {
    Join { //how come they don't have field name?, it's called variant
        chat_name: Arc<String>
    },
    Post { //post variant
        chat_name: Arc<String>,
        message: Arc<String>
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>
    },
    Error(String)
}