use std::{
    collections::HashMap,
    net::{Ipv6Addr, SocketAddr},
};

use clap::Parser;

fn parse_nat64_prefix(s: &str) -> Result<Ipv6Addr, String> {
    let prefix: Ipv6Addr = s
        .strip_suffix("/96")
        .ok_or("NAT64 prefix must be an IPv6 /96")?
        .parse()
        .map_err(|_| "Invalid IPv6 prefix")?;
    if u128::from(prefix) & u128::from(u32::MAX) != 0 {
        return Err("NAT64 prefix must have zero host bits".into());
    }
    Ok(prefix)
}

/// Parses key-value pairs in the format "KEY=VALUE,KEY2=VALUE2".
fn parse_key_val(s: &str) -> Result<HashMap<String, String>, String> {
    if s.trim().is_empty() {
        return Ok(HashMap::new());
    }

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
    #[arg(
        short,
        long,
        required_unless_present = "nat64_prefix",
        conflicts_with = "nat64_prefix"
    )]
    pub tailscale_site_id: Option<u16>,
    /// Synthesize addresses for a NAT64 translator instead of Tailscale 4via6.
    #[arg(long, value_parser = parse_nat64_prefix)]
    pub nat64_prefix: Option<Ipv6Addr>,
    /// Use TCP for both incoming queries and upstream lookups.
    #[arg(long)]
    pub tcp: bool,
    #[arg(
        short,
        long,
        default_value = "",
        value_parser = parse_key_val,
        value_name = "KEY=VALUE"
    )]
    pub cnames: HashMap<String, String>,
}
