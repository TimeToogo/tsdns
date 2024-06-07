use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: IpAddr,
    #[arg(short, long, default_value = "2053")]
    pub port: u16,
    #[arg(short, long)]
    pub tailscale_site_id: u16,
}
