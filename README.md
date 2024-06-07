tsdns
=====

Simple DNS proxy that maps private IPv4 to tailscale 4via6 IPv6 addresses.


## Usage

```
Usage: tsdns [OPTIONS] --tailscale-site-id <TAILSCALE_SITE_ID>

Options:
  -a, --address <ADDRESS>                      [default: 0.0.0.0]
  -p, --port <PORT>                            [default: 2053]
  -t, --tailscale-site-id <TAILSCALE_SITE_ID>  
  -h, --help                                   Print help
```

### Download

| Targets |
|---------|
| `x86_64-unknown-linux-gnu` | 
| `x86_64-unknown-linux-musl` |
| `aarch64-unknown-linux-gnu` | 
| `aarch64-unknown-linux-musl` |
| `aarch64-apple-darwin` |
| `x86_64-pc-windows-msvc` |

Example:

```sh
curl https://tsdns.elliotlevin.dev/tsdns-x86_64-unknown-linux-musl -o tsdns
chmod +x ./tsdns
./tsdns --help
```

### Acknowledgements

- [Tailscale](https://tailscale.com/) for a great product.
- [EmilHernvall](https://github.com/EmilHernvall/dnsguide) for the DNS parsing code.
