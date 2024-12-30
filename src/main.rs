#![allow(unused_imports)]

// base imports
use anyhow::Error;
use dexter_io_proxy::utils::tracer::init_tracing;
use scraper::{Html, Selector};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

// crate imports
use dexter_io_proxy::api::client::api_client;




#[tokio::main]
async fn main() {
    init_tracing();
    info!("Starting DEXTER-IO-PROXY API client");
    let api_futur = api_client();

    let _ = api_futur.await;
}
