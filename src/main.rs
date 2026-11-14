use std::net::{TcpListener, UdpSocket};

use clap::Parser;
use conf::Config;
use tsdns::*;

fn main() -> Result<()> {
    let conf = Config::parse();
    if conf.tcp {
        let listener = TcpListener::bind(conf.bind)?;
        // Bound concurrent connections while allowing pooled connections to be reused.
        std::thread::scope(|scope| {
            for _ in 0..8 {
                scope.spawn(|| {
                    for stream in listener.incoming() {
                        let result = stream
                            .map_err(Error::from)
                            .and_then(|mut stream| handle_tcp(&mut stream, &conf));
                        if let Err(err) = result {
                            eprintln!("TCP connection failed: {err}");
                        }
                    }
                });
            }
        });
        return Ok(());
    }
    let socket = UdpSocket::bind(conf.bind)?;

    loop {
        match handle_query(&socket, &conf) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
