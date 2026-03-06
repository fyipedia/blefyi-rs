# blefyi

[![crates.io](https://img.shields.io/crates/v/blefyi)](https://crates.io/crates/blefyi)
[![docs.rs](https://docs.rs/blefyi/badge.svg)](https://docs.rs/blefyi)

Async Rust client for the [BLEFYI](https://blefyi.com) API. Look up Bluetooth Low Energy chips, GATT profiles, beacon types, BLE versions, and manufacturer specifications.

## Install

```toml
[dependencies]
blefyi = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use blefyi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let results = client.search("nrf52").await?;
    println!("Found {} results", results.total);
    Ok(())
}
```

## API Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Search chips, profiles, and glossary |
| `chip(slug)` | Get BLE chip details |
| `profile(slug)` | Get GATT profile details |
| `version(slug)` | Get Bluetooth version details |
| `beacon(slug)` | Get beacon type details |
| `usecase(slug)` | Get use case details |
| `manufacturer(slug)` | Get manufacturer details |
| `glossary_term(slug)` | Get glossary term definition |
| `compare(slug_a, slug_b)` | Compare two BLE chips |
| `random()` | Get a random BLE chip |

All methods are async and return `Result<T, BleFyiError>`.

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| Python | [blefyi](https://pypi.org/project/blefyi/) | `pip install blefyi` |
| TypeScript | [blefyi](https://www.npmjs.com/package/blefyi) | `npm install blefyi` |
| Go | [blefyi-go](https://pkg.go.dev/github.com/fyipedia/blefyi-go) | `go get github.com/fyipedia/blefyi-go` |
| Rust | [blefyi](https://crates.io/crates/blefyi) | `cargo add blefyi` |
| Ruby | [blefyi](https://rubygems.org/gems/blefyi) | `gem install blefyi` |

## Code FYI Family

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode symbologies & standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types & encoding |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips & protocols |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags & readers |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart card platforms |

## License

MIT
