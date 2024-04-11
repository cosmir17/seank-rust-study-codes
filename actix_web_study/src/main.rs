use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// example from zero to production in rust
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    }).bind("127.0.0.1:8000")?
        .run()
        .await
}

//curl http://127.0.0.1:8000
//Hello World!%
//where did the %sign come from?

//cargo check

// In Rust, the impl Trait syntax is used to return a value that implements a specific trait without explicitly specifying the concrete type. In this case, impl Responder means that the greet function returns a value that implements the Responder trait.
//
// The Responder trait is part of the actix-web framework and is used to define how a value should be converted into an HTTP response. By returning impl Responder, the greet function is indicating that it will return a value that can be converted into an HTTP response, but the exact type of that value is not specified.
//
// This is useful because it allows the greet function to return different types that all implement the Responder trait, providing flexibility in the return type. For example, the greet function could return a String, a web::Json value, or any other custom type that implements the Responder trait.
//
// Using impl Responder as the return type also helps to keep the function signature concise and avoids exposing the concrete return type in the function signature. The actual type returned by the function is determined by the implementation of the function body.
//
// In the given code, the greet function returns a String wrapped in format!(), which implements the Responder trait. This allows the string to be directly returned as the HTTP response body.
//
// So, while impl Responder may look unusual compared to specifying a concrete type, it is a powerful feature in Rust that allows for flexibility and abstraction in the return type of a function.
//
// Copy
//
//
// S
// I sent curl request
//
//
// Copy code
// curl http://127.0.0.1:8000
//
// Copy code
// Hello World!%
//
// where did the %sign come from?
// The % sign at the end of the response is likely due to the lack of a proper newline character at the end of the string returned by the greet function.
//
// In the greet function, the response string is created using format!("Hello {}!", &name). However, this string does not include a newline character at the end. When the response is sent back to the client (in this case, curl), the % sign is displayed to indicate that the line does not end with a newline character.
//
// To fix this, you can modify the greet function to include a newline character at the end of the response string. Here's the modified code:
//
// rust
//
//
// Copy code
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!\n", &name)
// }
// By adding \n at the end of the string, the response will include a proper newline character, and the % sign will not appear when making a request with curl.
//
// After making this change, if you run the server again and send the curl request:
//
//
// Copy code
// curl http://127.0.0.1:8000
// You should see the response without the % sign:
//
//
// Copy code
// Hello World!
// The newline character ensures that the response ends with a proper line ending, providing a cleaner output in the console.