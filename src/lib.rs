use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn cloud_start(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let body = include_str!("index.html");

    Ok(http::Response::builder()
        .status(200)
        .body(Some(body.into()))?)
}
