# m4rs - Moving Average for Rust

[![crates.io](https://img.shields.io/crates/v/m4rs)](https://crates.io/crates/m4rs)

- Trading indicator library
- Small and simple implementation
- No extra dependencies
- Supports following indicators
    - ATR
    - Awesome Oscillator
    - Bolinger Band
    - CCI
    - DEMA
    - DMI/ADX
    - EMA
    - Envelope
    - Heikin Ashi
    - HMA
    - Ichimoku Kinko Hyo
    - MACD
    - Momentum
    - Parabolic SAR
    - RCI
    - RMA
    - RSI
    - SMA
    - Standard Deviation
    - Stochastics (Fast, Slow)
    - TEMA
    - VWMA
    - Williams %R
    - WMA
- Call it "Mars"

# Installation

```sh
cargo add m4rs
```

# Examples

```rust
// Prepare candlesticks in some way such as by retrieving them from the exchange's API
// And make them into m4rs::Candlestick objects
let entries: Vec<m4rs::Candlestick> = vec![
    (1719400001, 100.0, 130.0, 90.0, 110.0, 1000.0),
    (1719400002, 110.0, 140.0, 100.0, 130.0, 1000.0),
    (1719400003, 130.0, 135.0, 120.0, 120.0, 1000.0),
    (1719400004, 120.0, 130.0, 80.0, 90.0, 1000.0),
    (1719400005, 90.0, 100.0, 70.0, 80.0, 1000.0),
    (1719400006, 80.0, 180.0, 60.0, 120.0, 1000.0),
    (1719400007, 120.0, 210.0, 110.0, 180.0, 1000.0),
    (1719400008, 180.0, 185.0, 170.0, 180.0, 1000.0),
    (1719400009, 180.0, 220.0, 140.0, 200.0, 1000.0),
]
.iter()
.map(|(at, o, h, l, c, v)| m4rs::Candlestick::new(*at, *o, *h, *l, *c, *v))
.collect();

// Get 3SMA calculation result
let result = m4rs::sma(&entries, 3).unwrap();

for x in &result {
    println!("{}: {:.1}", x.at, x.value);
}
// 1719400003: 120.0
// 1719400004: 113.3
// 1719400005: 96.7
// 1719400006: 96.7
// 1719400007: 126.7
// 1719400008: 160.0
// 1719400009: 186.7
```

# API Reference

- https://docs.rs/m4rs/latest/m4rs/
