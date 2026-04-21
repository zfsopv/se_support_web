mod app;
mod config;
mod faq_parser;
mod judge;
mod matrix;
mod models;
mod report;

use anyhow::Result;
use clap::Parser;
use config::{CliArgs, EnvConfig};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    dotenv::dotenv().ok();

    let cli_args = CliArgs::parse();
    let env_config = EnvConfig::from_env()?;

    app::run(cli_args, env_config).await
}
