use async_std::net;
use async_std::task;
use chat_program_study::utils::ChatResult;
use std::sync::Arc;
use async_std::prelude::*;

mod connection;
mod chats;
mod chats_map;

use connection::handle;

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        println!("Error: {}", error);
    }
}

// the server(main.rs) is responsible for receiving incoming connections from clients and handling their requests.
// it's composed of four files
// main is the entry point of the server and is responsible for setting up the servers listening socket (net::TcpListener::bind(addr).await?;) as well as
// accepting incoming connections (new_connections = listener.incoming();)
// spawn new threads (async_std::task::block_on(async {) for each incoming connections
// each thread created in main is responsible for processing incoming requests from single clients

// connection.rs is used to represent a single connection between the server and a client.
// it contains methods to read and write data to and from the client, as well as to handle specific requests
// such as sending messages to the chat room (handle method)

// chats.rs contains the logic for managing our chat rooms. It provides methods for creating new chat rooms
// and sending messages to the chat rooms

// chats_maps.rs which is responsible for maintaining and mapping of our chat room IDs to the actual chat objects.

//client.rs is responsible for connecting to the server and sending requests to it.
//it's not doing too much. we are just trying to connect to our server and then send messages to a specified chat room and then

//server and client both utilise lib.rs and utils.rs modules

//lib.rs contains our common functionality shared between the server and the client, such as
//the message structures and the error handling

//utils.rs contains helper functions used by both the server and client, such as functions for serializing and deserializing data

//so overall to summarize this, the client and the server work together by establishing a connection
//to each other with the client, sending requests to the server and the server responding to those requests.
//the server manages the chat rooms using the chats and the chats_map modules while the client sends requests
//to join chat rooms and sends messages using the client module.

//cargo run --release --bin server localhost:8080

fn main() -> ChatResult<()> { //what's the significance of returning something out of main function???
    let addr = std::env::args().nth(1).expect("Server ADDRESS");

    let chat_table = Arc::new(chats_map::ChatTracker::new());
    //we are creating a shared thread, safe data structure to store our chat rooms and our chat table.
    //so another words, this means that we're creating a thread, safe reference counting pointer that can
    //shared across multiple thread

    //we want to start the server and we'll start it using an async standard
    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(addr).await?;
        let mut new_connections = listener.incoming();
        //this is going to create a stream of incoming TCP connections by calling the incoming method on the TCP listner

        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?; //this is going to unwrap the next connection in the stream and which will produce a TCP
            //stream if it was a successful connection or it's going to propagate any
            let chats = chat_table.clone(); //this clones the chat tracket data structure so that the connection
            // handler can access the same chat tracker instance as other handlers running concurrently.

            task::spawn(async {//this spawns a new asyc task that calls the handle function defined in the connection
                // module to handle the incoming connections
                log_error(handle(socket, chats).await); //log error logs an error happens during handling of the connection
            }); //task spawn is creating a new task to execute the future, which allows the program
            // to handle multiple connections concurrently for us
        }
        Ok(())
    })
}

