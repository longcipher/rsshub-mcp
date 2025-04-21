#![allow(unused)]
use std::path::PathBuf;

use clap::Parser;
use config::{Config as FileConfig, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Clone, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub config: Option<PathBuf>,
    #[clap(short, long, default_value = "false")]
    pub version: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub sse_server_addr: String,
}

impl Config {
    pub fn new(config: Option<PathBuf>) -> Result<Self, ConfigError> {
        let c = FileConfig::builder()
            .add_source(File::from(config.expect("Config file not found")))
            .add_source(Environment::with_prefix("RSSHUB_MCP"))
            .build()?;
        c.try_deserialize()
    }
}
