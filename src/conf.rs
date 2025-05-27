use std::{collections::HashMap, net::SocketAddr};

use clap::Parser;

/// Parses key-value pairs in the format "KEY=VALUE,KEY2=VALUE2".
fn parse_key_val(s: &str) -> Result<HashMap<String, String>, String> {
    s.split(',')
        .map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Ok((key.to_string(), value.to_string())),
                _ => Err(format!("Invalid key-value pair: {}", pair)),
            }
        })
        .collect::<Result<HashMap<_, _>, _>>()
        .map_err(|e| e.to_string())
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(short, long, default_value = "0.0.0.0:2053")]
    pub bind: SocketAddr,
    #[arg(short, long)]
    pub upstream: SocketAddr,
    #[arg(short, long)]
    pub tailscale_site_id: u16,
    #[arg(short, long, value_parser = parse_key_val, value_name = "KEY=VALUE")]
    pub cnames: HashMap<String, String>,
}
