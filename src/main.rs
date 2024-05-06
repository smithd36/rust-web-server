/**
 * Author: ds
 * Date: 2021
 * A basic rust web server.
 */
use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};
use std::{convert::Infallible, net::SocketAddr};

async fn service_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Logic here is guaranteed not to fail because of 'Infallible' type
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 200], 3000));
    let server = Server::bind(&addr).serve(make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(service_handler)) }
    }));

    if let Err(e) = server.await {
        eprintln("server error: {}", e)
    }
}