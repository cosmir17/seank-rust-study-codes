// use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{task, io, net};
use std::sync::Arc;

use chat_program_study::utils::{self, ChatResult};
use chat_program_study::{Client, Server};

fn get_value(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),
        None => Some((input, ""))
    }
}

fn parse_input(line: &str) -> Option<Client> {
    let (input, remainder) = get_value(line)?;

    if input == "join" {
        let (chat, remainder) = get_value(remainder)?;

        if !remainder.trim_start().is_empty() {
            return None;
        }

        return Some(Client::Join {chat_name: Arc::new(chat.to_string())});
    }

    else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        let message = remainder.trim_start().to_string();

        return Some(Client::Post{ chat_name: Arc::new(chat.to_string()), message: Arc::new(message)});
    }

    else {
        println!("Unrecognised input: {:?}", line);
        return None;
    }
}

async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Options: \nJoin CHAT\npost CHAT MESSAGE");

    let mut options = io::BufReader::new(io::stdin()).lines();

    while let Some(option_result) = options.next().await {
        let opt = option_result?;
        let req = match parse_input(&opt){
            Some(req) => req,
            None => continue
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}

async fn messages(server: net::TcpStream) -> ChatResult<()> {
    let buf = io::BufReader::new(server);
    let mut stream = utils::receive(buf);

    while let Some(msg) = stream.next().await {
        match msg? {
            Server::Message { chat_name, message } => {
                println!("Chat Name: {}\n, Message: {}\n", chat_name, message);
            }
            Server::Error(message) => {
                println!("Error received: {}", message);
            }
        }
    }
    Ok(())
}

// cargo run --release --bin client localhost:8080
fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address:PORT"); //reading from terminal for server's ip address and port it's listening to

    task::block_on(async {
        let socket = net::TcpStream::connect(addr).await?; //connect on the server using the address and the port
        socket.set_nodelay(true)?; //it's going to disable Nagle's algorithm to reduce the latency
        //segments are always sent as soon as possible, even if there's only small amount of data, wbhen it's not set, the data is buffered
        //until there is a sufficient amount of data to send out, thereby avoiding the frequent sending of packets.
        //true : send us data asap that way our client are recieing the messageas fast as can, thus avoid any unnessary delays

        let send = send(socket.clone()); // create a new task to send a new message to server
        let replies = messages(socket); // to recieve the message to the server

        replies.race(send).await?; //to race each other, it allows the two tasks, send and replies to run concurrently and then we're waiting for
        //one of them to complete, either send or replies to complete first, when that happens, we do our logic from there
        //in the mean time, we just want to see who completes first, so we do 'race' each other
        Ok(())
    })
}