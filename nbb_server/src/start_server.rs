use axum::Router;
use hyper::body::Incoming;
use hyper::Request;
use hyper_util::rt::{TokioExecutor, TokioIo};
use nbb_config::ServerBindType;
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;
use tower::Service;

pub async fn start_server() {
    let router = crate::router::build_router();
    let cfg = nbb_config::get_config();

    if cfg.general.cache_rendered_pages && !cfg.general.render_on_request {
        nbb_markdown::prerender_all(cfg.general.data_dir.as_ref());
    }

    match cfg.server.bind_address {
        ServerBindType::Tcp(ref addr, port) => start_server_tcp(addr, port, router).await,
        ServerBindType::Unix(ref path) => start_server_uds(path, router).await,
    }
}

async fn start_server_tcp(addr: &str, port: u16, router: Router) {
    let addr = SocketAddr::new(IpAddr::from_str(addr).expect("invalid bind address"), port);
    let tcp = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port");
    axum::serve(tcp, router)
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
    let listener = tokio::net::UnixListener::bind(path).expect("failed to bind to unix socket");

    let mut make_service = router.into_make_service();

    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();

        let tower_service = unwrap_infallible(make_service.call(&socket).await);

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);

            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().call(request)
            });

            if let Err(err) = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}

fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => match err {},
    }
}
