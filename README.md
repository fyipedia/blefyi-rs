# blefyi

[![crates.io](https://img.shields.io/crates/v/blefyi.svg)](https://crates.io/crates/blefyi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust client for the [BLEFYI](https://blefyi.com) REST API. BLE, GATT, beacons. Uses `reqwest` for HTTP.

> **Explore at [blefyi.com](https://blefyi.com)** — interactive tools and comprehensive reference.

## Install

```toml
[dependencies]
blefyi = "0.1"
```

## Quick Start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = blefyi::Client::new();
    let result = client.search("query")?;
    println!("{:?}", result);
    Ok(())
}
```

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **Python** | `pip install blefyi` | [PyPI](https://pypi.org/project/blefyi/) |
| **npm** | `npm install blefyi` | [npm](https://www.npmjs.com/package/@fyipedia/blefyi) |
| **Go** | `go get github.com/fyipedia/blefyi-go` | [pkg.go.dev](https://pkg.go.dev/github.com/fyipedia/blefyi-go) |
| **Rust** | `cargo add blefyi` | [crates.io](https://crates.io/crates/blefyi) |
| **Ruby** | `gem install blefyi` | [rubygems](https://rubygems.org/gems/blefyi) |


## Links

- **Site**: [blefyi.com](https://blefyi.com)
- **API**: [blefyi.com/api/v1/](https://blefyi.com/api/v1/)
- **OpenAPI**: [blefyi.com/api/v1/schema/](https://blefyi.com/api/v1/schema/)

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## License

MIT
