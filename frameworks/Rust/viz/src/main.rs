#![allow(clippy::unused_async)]

use serde::Serialize;
use viz::{
    header::{HeaderValue, SERVER},
    Bytes, Error, Request, Response, ResponseExt, Result, Router,
};

mod server;
mod utils;

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

async fn plaintext(_: Request) -> Result<Response> {
    let mut res = Response::text("Hello, World!");
    res.headers_mut()
        .insert(SERVER, HeaderValue::from_static("Viz"));
    Ok(res)
}

async fn json(_: Request) -> Result<Response> {
    let mut res = Response::with(
        http_body_util::Full::new(Bytes::from(
            serde_json::to_vec(&Message {
                message: "Hello, World!",
            })
            .unwrap(),
        )),
        mime::APPLICATION_JSON.as_ref(),
    );
    res.headers_mut()
        .insert(SERVER, HeaderValue::from_static("Viz"));
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .get("/plaintext", plaintext)
        .get("/json", json);

    server::serve(app).await.map_err(Error::Boxed)
}
