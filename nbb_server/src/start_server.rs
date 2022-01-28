use axum::Router;
use nbb_config::ServerBindType;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::signal::ctrl_c;

pub async fn start_server() {
    let router = crate::router::build_router();
    let cfg = nbb_config::get_config();

    match cfg.server.bind_address {
        ServerBindType::Tcp(ref addr, port) => start_server_tcp(addr, port, router).await,
        ServerBindType::Unix(ref path) => start_server_uds(path, router).await,
    }
}

async fn start_server_tcp(addr: &str, port: u16, router: Router) {
    let addr = SocketAddr::new(IpAddr::from_str(addr).expect("invalid bind address"), port);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            ctrl_c().await.expect(
                "failed to wait for ctrl+c:\
                 you will need to SIGTERM the server if you want it to shut down",
            );
        })
        .await
        .expect("failed to bind and serve over TCP");
}

async fn start_server_uds(path: &str, router: Router) {
    let listener = tokio::net::UnixListener::bind(&path).expect("failed to bind to unix socket");
    let stream = tokio_stream::wrappers::UnixListenerStream::new(listener);
    let acceptor = hyper::server::accept::from_stream(stream);

    axum::Server::builder(acceptor)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            ctrl_c().await.expect(
                "failed to wait for ctrl+c:\
                 you will need to SIGTERM the server if you want it to shut down",
            );
        })
        .await
        .expect("failed to bind and serve over UDS");
}
