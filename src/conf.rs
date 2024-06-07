use std::net::SocketAddr;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(short, long, default_value = "0.0.0.0:2053")]
    pub bind: SocketAddr,
    #[arg(short, long)]
    pub upstream: SocketAddr,
    #[arg(short, long)]
    pub tailscale_site_id: u16,
}
