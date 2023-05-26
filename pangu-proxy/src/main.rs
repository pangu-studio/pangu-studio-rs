use std::error::Error;
use hyper::{body::HttpBody, Client};
use hyperlocal::{UnixClientExt, Uri};
use tokio::io::{self, AsyncWriteExt as _};

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use reverse_proxy::ReverseProxy;
use hyper_trust_dns::{RustlsHttpsConnector, TrustDnsResolver};
use hyperlocal::{UnixConnector};
// use hyper_trust_dns::{RustlsHttpsConnector, TrustDnsResolver};
use std::net::IpAddr;
use std::{convert::Infallible, net::SocketAddr};

mod reverse_proxy;
#[macro_use]
extern crate tracing;


lazy_static::lazy_static! {
    static ref  PROXY_CLIENT_UNIX: ReverseProxy<UnixConnector> = {
        ReverseProxy::new(
            hyper::Client::unix(),
        )
    };
    static ref  PROXY_CLIENT: ReverseProxy<RustlsHttpsConnector> = {
        ReverseProxy::new(
            hyper::Client::builder().build::<_, hyper::Body>(TrustDnsResolver::default().into_rustls_webpki_https_connector()),
        )
    };
}

fn debug_request(req: &Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

async fn handle(client_ip: IpAddr, mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    print!("client_ip: {:?}, req: {:?}\n", client_ip, req);
    if req.uri().path().starts_with("/unix_test/") {
        // req.uri().path().split("/").nth(2)
        let path = req.uri().path().replace("/unix_test/", "/");
        *req.uri_mut() = path.parse().unwrap();
        match PROXY_CLIENT_UNIX.call(client_ip, "/Users/liwei/.lima/docker/sock/docker.sock", req)
            .await
        {
            Ok(response) => {
                Ok(response)
            },
            Err(_error) => {
                Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap())},
        }
    } 
    // else if req.uri().path().starts_with("/target/second") {
    //     match PROXY_CLIENT.call(client_ip, "http://127.0.0.1:13902", req)
    //         .await
    //     {
    //         Ok(response) => Ok(response),
    //         Err(_error) => Ok(Response::builder()
    //             .status(StatusCode::INTERNAL_SERVER_ERROR)
    //             .body(Body::empty())
    //             .unwrap()),
    //     }
    // } 
    
    else {
        debug_request(&req)
    }
}

#[tokio::main]
async fn main() {
    let bind_addr = "127.0.0.1:8000";
    let addr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle(remote_addr, req))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

















// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     let url = Uri::new("/Users/liwei/.lima/docker/sock/docker.sock", "/test?ab=1").into();

//     print!("url: {:?}", url);

//     let client = Client::unix();

//     let mut response = client.get(url).await?;

//     while let Some(next) = response.data().await {
//         let chunk = next?;
//         io::stdout().write_all(&chunk).await?;
//     }

//     Ok(())
// }