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