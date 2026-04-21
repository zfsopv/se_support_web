use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about = "Matrix FAQ answer accuracy test tool")]
pub struct CliArgs {
    #[arg(long, help = "Path to a FAQ markdown file")]
    pub faq_file: PathBuf,

    #[arg(long, help = "Optional output directory for JSON and CSV reports")]
    pub output_dir: Option<PathBuf>,

    #[arg(long, help = "Optional limit for how many FAQ questions to run")]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub homeserver: String,
    pub test_user: String,
    pub test_password: String,
    pub openai_api_key: String,
    pub openai_model: String,
    pub openai_base_url: String,
    pub reply_timeout_seconds: u64,
    pub sync_timeout_ms: u64,
    pub output_dir: PathBuf,
}

impl EnvConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            homeserver: required_var("HOMESERVER")?,
            test_user: required_var("TEST_USER")?,
            test_password: required_var("TEST_PASSWORD")?,
            openai_api_key: required_var("OPENAI_API_KEY")?,
            openai_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".to_string()),
            openai_base_url: env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            reply_timeout_seconds: optional_u64("REPLY_TIMEOUT_SECONDS", 120)?,
            sync_timeout_ms: optional_u64("SYNC_TIMEOUT_MS", 30_000)?,
            output_dir: PathBuf::from(
                env::var("OUTPUT_DIR").unwrap_or_else(|_| "./reports".to_string()),
            ),
        })
    }
}

fn required_var(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("Missing required environment variable {name}"))
}

fn optional_u64(name: &str, default: u64) -> Result<u64> {
    match env::var(name) {
        Ok(value) => value
            .parse::<u64>()
            .with_context(|| format!("Invalid integer value for {name}: {value}")),
        Err(_) => Ok(default),
    }
}
