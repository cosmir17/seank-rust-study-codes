use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};
use chat_program_study::utils::{self, ChatResult};
use chat_program_study::{Client, Server};
use crate::chats_map::ChatTracker;

pub struct Leaving(Mutex<TcpStream>);
//the leaving struct represents an outbound TCP stream
//when created, the leaving value takes ownership of a tcp stream and wraps it in a mutex to ensure that
//one task can use it at a time

impl Leaving { //so the new function creates a new leaving instance with a mutex protected tcp stream and the send
    //function sends a server packet to the client over that stream. the lock variable represents a locked mutex protected
    //tcp stream, and the send json function sends  json encoded string to the licnet and then we immediately flush it
    //away to ensure that any buffered data is sent immediately
    pub fn new(client: TcpStream) -> Leaving {
        Leaving(Mutex::new(client))
    }

    pub async fn send(&self, packet: Server) -> ChatResult<()> { //this right here, server packet (packet: Server)
        let mut lock = self.0.lock().await; //mutex protected tcp stream

        utils::send_json(&mut *lock, &packet).await?; //send the sever packet as a json encoded string to the client (await to finish)

        lock.flush().await?; //flush any remaining buffered data to ensure that it is sent immediately

        Ok(())
    }
}

pub async fn handle(socket: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {
    let leaving = Arc::new(Leaving::new(socket.clone()));

    let buffered = BufReader::new(socket);
    let mut from_client = utils::receive(buffered);
    //this is creating a new stream of client requests by calling the receive function from the util modules and passing
    //it the buffered input

    while let Some(req_res)  = from_client.next().await {
        let request = req_res?;

        let result = match request {
            Client::Join { chat_name } => {
                let chat = chats.find_or_new(chat_name);
                chat.join(leaving.clone());
                Ok(())
            }
            Client::Post { chat_name, message } => match chats.find(&chat_name) {
                Some(chat) => {
                    chat.post(message);
                    Ok(())
                }
                None => Err(format!("Chat does not exist: {}", chat_name)),
            },
        };

        if let Err(message) = result {
            let report = Server::Error(message);
            leaving.send(report).await?;
        }
    }
    Ok(())
}
