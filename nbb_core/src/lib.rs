use tokio::runtime::Runtime;
use tracing_subscriber::EnvFilter;

fn load_config() {
    let cfg_location = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "./config.yaml".to_string());
    nbb_config::load_config(cfg_location);
}

fn init_tracing() {
    let env_filter = match nbb_config::get_config().server.log_filter {
        Some(ref filter) => EnvFilter::from(filter),
        None => EnvFilter::from_default_env(),
    };
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

fn get_tokio_rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build rt")
}

pub fn start() {
    load_config();
    init_tracing();
    let rt = get_tokio_rt();
    rt.block_on(nbb_server::start_server());
}
