# LPLang — HTTP Server

use std::net::http::{Server, Request, Response}

async fn handler(req: Request) -> Response {
    let name = req.query("name").unwrap_or("world");
    Response::ok(format!("Hello, {name}!"))
}

async fn main() -> Result<(), Error> {
    let server = Server::bind("127.0.0.1:8080").await?;
    print("Server running on http://127.0.0.1:8080")
    server.serve(handler).await?;
    Ok(())
}