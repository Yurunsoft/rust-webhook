use config::get_config;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

mod config;
mod gitee;
mod github;
mod route;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = &get_config()?;

    let addr_string = format!("{}:{}", config.server.host, config.server.port);
    let addr: SocketAddr = addr_string.parse().expect("Unable to parse socket address");

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(route::route)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started!");
    println!("Listen: {}", addr_string);
    drop(config);
    drop(addr_string);
    server.await?;
    Ok(())
}
