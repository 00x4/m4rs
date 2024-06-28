# m4rs

- Moving Average indicator library for Rust
- Small and simple implementation
- No extra dependencies
- Supports following indicators
    - ATR
    - Bolinger Band
    - EMA
    - Ichimoku Kinko Hyo
    - MACD
    - RMA
    - RSI
    - SMA
    - Stochastics
- Call it "Mars"

# Installation

```sh
cargo add m4rs
```

# Examples

```rust
// Prepare candlesticks in some way
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
let result = m4rs::sma(&entries, 3);

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
