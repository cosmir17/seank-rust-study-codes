use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::chats::Chats;

pub struct ChatTracker(Mutex< HashMap<Arc<String>, Arc<Chats>> >); //has a mutex hashmap arc string arc chat field
//map from the chat room names to the actual chat instances, keep track of all of our chat rooms

impl ChatTracker {
    pub fn new() -> ChatTracker {
        ChatTracker( Mutex::new(HashMap::new()) )
    }

    pub fn find(&self, name: &String) -> Option<Arc<Chats>> { //take in a string reference name and then we need to return arc reference to the
        // chat instance associated with that name
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn find_or_new(&self, name: Arc<String>) -> Arc<Chats> {
        self.0.lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Chats::new(name)))
            .clone()
    }
}