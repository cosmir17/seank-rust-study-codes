use async_std::task;
use crate::connection::Leaving;
use std::sync::Arc;
use tokio::sync::broadcast; //tokio is a crate for writing reliable, async and multithreaded rust applications
//it provides tools for tasks, networking and input and output and allows rust programs to run efficiently
//on modern hardware architecture
//Tokio is built on top of the async and await language features in Rust and it provides us a powerful
//and scalable foundation for building high performance network services and applications

//so tokio is a great tool to use when building async utilities for networking and IO

use chat_program_study::Server;
use tokio::sync::broadcast::error::RecvError;

pub struct Chats { //a chatroom that contains chats?
    name: Arc<String>,
    publisher: broadcast::Sender<Arc<String>> //broadcasting channel that sends reference counting pointer across the broadcast
}

impl Chats {
    pub fn new(name: Arc<String>) -> Chats { //name is chatroom name
        let (publisher, _) = broadcast::channel(1000); //broadcast sender for sending messages with buffer size of 1000 messages
        Chats {name, publisher}
    }

    pub fn join(&self, leaving: Arc<Leaving>) {
        let receiver = self.publisher.subscribe();
        task::spawn(sub(self.name.clone(), receiver, leaving)); //this spawns a new task that listens for new messages
    }

    pub fn post(&self, message: Arc<String>) { //send message to the publisher, this method takes in another arc string
        //and it's going to represent a new message to be broadcasted to all of the chat members
        let _ = self.publisher.send(message);
    }
}

async fn sub(chat_name: Arc<String>, mut receiver: broadcast::Receiver<Arc<String>>, leaving: Arc<Leaving>) {//what it does is it's going to be the method to send and receive chat messages to our members
    loop { //this function is going to contain an infinite loop that waits for incoming messages on the
        //broadcast receiver and we are going to use the recieve method
        //we are going to block the current task until a message is received
        //once a message is receieved, it's going to match the result using a match statement and create a new server message struct with the chat room name and message if the message was successsfully recieved.
        //otherwise, we have RecvError
        //otherwise, if get an error code, then we have some message that we're going to need to create to send to the chat room.

        let packet = match receiver.recv().await {
            Ok(message) => Server::Message {
                chat_name: chat_name.clone(),
                message: message.clone()
            },
            Err(RecvError::Lagged(n)) => {
                Server::Error(format!("Dropped {} message from {}.", n, chat_name))
            },
            Err(RecvError::Closed) => break, //because the channel is closed, we need to get out of this loop because the chat no longer exists
        };

        if leaving.send(packet).await.is_err() { //getting error, we need to break out the loop as well
            break;
        }
    }

}