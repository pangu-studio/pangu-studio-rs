use hyper::client::HttpConnector;
use hyperlocal::UnixClientExt;
use std::time::Duration;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper_trust_dns::{RustlsHttpsConnector, TrustDnsResolver};
use hyperlocal::UnixConnector;
use reverse_proxy::{ClientType, ReverseProxy};
use std::net::IpAddr;
use std::{convert::Infallible, net::SocketAddr};

mod conf;
mod reverse_proxy;

lazy_static::lazy_static! {
    static ref  PROXY_CLIENT_UNIX: ReverseProxy<UnixConnector> = {
        ReverseProxy::new(
            hyper::Client::unix(),
        )
    };
    static ref  PROXY_CLIENT_HTTPS: ReverseProxy<RustlsHttpsConnector> = {
        ReverseProxy::new(
            hyper::Client::builder().build::<_, hyper::Body>(TrustDnsResolver::default().into_rustls_webpki_https_connector()),
        )
    };
    static ref  PROXY_CLIENT: ReverseProxy<HttpConnector> = {
        ReverseProxy::new(
             hyper::Client::builder()
    .pool_idle_timeout(Duration::from_secs(30))
    // .http2_only(true)
    .build_http(),
        )
    };
}

fn debug_request(req: &Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

async fn handle(client_ip: IpAddr, mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // check token
    let token = req.headers().get("x-api-key");
    if token.is_none() || token.unwrap() != &conf::CONFIG.secret {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap());
    }
    debug!("secret validated: {:?}\n", token);
    let rule = conf::CONFIG
        .rules
        .iter()
        .find(|rule| req.uri().path().starts_with(&rule.prefix));

    debug!("rule: {:?}\n", rule);
    match rule {
        Some(rule) => {
            debug!("rule matched: {:?}\n", rule);

            debug!("rule: {:?}\n", rule);
            let path = req.uri().path().replace(&rule.prefix, "");
            *req.uri_mut() = path.parse().unwrap();
            match rule.schema.as_str() {
                "unix" => {
                    match PROXY_CLIENT_UNIX
                        .call(client_ip, ClientType::UNIX, &rule.target, req)
                        .await
                    {
                        Ok(response) => Ok(response),
                        Err(_error) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
                "http" => {
                    match PROXY_CLIENT
                        .call(client_ip, ClientType::TCP, &rule.target, req)
                        .await
                    {
                        Ok(response) => Ok(response),
                        Err(_error) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
                "https" => {
                    match PROXY_CLIENT_HTTPS
                        .call(client_ip, ClientType::TCP, &rule.target, req)
                        .await
                    {
                        Ok(response) => Ok(response),
                        Err(_error) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
                _ => {
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap());
                }
            }
        }
        None => {
            debug_request(&req).unwrap();
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        }
    }
}

#[macro_use]
extern crate log;

use env_logger::Env;

#[tokio::main]
async fn main() {
    //init logger
    let env = Env::default()
        .filter_or("PS_LOG_LEVEL", conf::CONFIG.logging.level.as_str())
        .write_style_or("PS_LOG_STYLE", conf::CONFIG.logging.write_style.as_str());

    env_logger::init_from_env(env);
    debug!(
        r#"config:[
    listen: {:?}
    {:?}
    {:?}
]"#,
        conf::CONFIG.listen,
        conf::CONFIG.logging,
        conf::CONFIG.rules
    );
    info!("Starting server...");
    let bind_addr = &conf::CONFIG.listen;
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
