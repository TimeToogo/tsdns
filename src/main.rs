use std::net::UdpSocket;

use clap::Parser;
use conf::Config;
use tsdns::*;

fn main() -> Result<()> {
    let conf = Config::parse();
    let socket = UdpSocket::bind(conf.bind)?;

    loop {
        match handle_query(&socket, &conf) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
