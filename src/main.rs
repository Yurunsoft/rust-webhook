use std::convert::Infallible;
use std::net::SocketAddr;
use config::get_config;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};

mod route;
mod gitee;
mod github;
mod config;
mod util;

#[tokio::main]
async fn main() {
    let config = &get_config();

    let addr: SocketAddr = (config.server.host.clone() + ":" + &config.server.port.to_string())
        .parse()
        .expect("Unable to parse socket address");

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(route::route))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started!");
    // let tmp_config = &config;
    println!("Listen: {}:{}", config.server.host, config.server.port);
    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
