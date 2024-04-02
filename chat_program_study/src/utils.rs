use async_std::prelude::*;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
//Box : a rust smart pointer that's going to provide heap allocation and reference
// counting value of specific type. Box is often used to provide a level of indirection and avoid large amount
// of copying when dealing with complex data structures
//
//dyn error : is a trait that represents an error type that can be used within the result type
// this trait defines the behavior of the error types that can be thrown and caught by Rust's error handling mechanisms
//
//Send and Sync : they are rust's marker traits that indicate that a type is safe to send or share between different threads
//static life time ('static) : this indicates that the error type must have a life time that is at least as long as the entire lifetime of the program

pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize
{
    let mut json = serde_json::to_string(packet)?; //serialise the packet arg to json string and return fail if it fails
    json.push('\n'); //

    leaving.write_all(json.as_bytes()).await?; //writing the json string for the leaving arg if that fails.
    // if nothing fails, we successuflly completed sending json, therefore, we return a json result
    Ok(())
}

pub fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
    I: async_std::io::BufRead + Unpin, // read bytes asyncrously and can be safely unpinned
    T: DeserializeOwned //T generic to implement DeserializeOwned trait and data strucutre that can be deserised without borrowing any data structures from the deserilizer
//so this is primarily useful for trait bounds for functions such as from string
{
    incoming.lines().map(|line| -> ChatResult<T> { //stream of incoming to be lines -> line, map line to ChatResult<T>
        let li = line?;
        let msg = serde_json::from_str::<T>(&li)?; //deserialising the line to a chat message of type T and returning an error if it fails
        Ok(msg) //if we haven't failed this point, then we can say, 'ok' on the message, successful result containing the chat message
    })
}