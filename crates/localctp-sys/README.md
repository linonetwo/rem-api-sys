# 本地期货交易所 LocalCTP 的 Rust 绑定

## 1. `simulate_market_data`

### 功能描述

`simulate_market_data` 函数用于模拟市场数据并插入到交易系统中。它生成虚假的市场报价数据，并在指定的时间间隔内定期插入这些数据。

### 函数签名

```rust
async fn simulate_market_data(api: &mut CThostFtdcTraderApi)
```

### 参数

- `api`: `&mut CThostFtdcTraderApi` 类型，这是一个 CTP 交易 API 的可变引用，用于插入模拟的市场数据。

### 主要逻辑

1. **时间间隔控制**：使用 `time::interval(Duration::from_millis(500))` 创建一个每 500 毫秒触发一次的定时器。
2. **随机数生成器**：使用 `StdRng::from_entropy()` 创建一个线程安全的随机数生成器。
3. **循环生成数据**：在循环中，每次间隔触发时，生成一条 `FakeMarketQuote` 类型的市场报价数据。
4. **插入市场数据**：调用 `api.insert_market_quote(market_quote)` 方法将生成的市场报价数据插入到交易系统中。如果插入失败，打印错误信息。

### 数据结构

#### `FakeMarketQuote`

`FakeMarketQuote` 是一个模拟的市场报价数据结构，包含以下字段：

- `instrument_id`: `String` 类型，合约代码。
- `bid_price`: `f64` 类型，买价。
- `ask_price`: `f64` 类型，卖价。
- `quote_ref`: `String` 类型，报价参考。
- `last_price`: `String` 类型，最新价。
- `settlement_price`: `String` 类型，结算价。
- `upper_limit_price`: `String` 类型，涨停价。
- `lower_limit_price`: `String` 类型，跌停价。
- `business_unit`: `String` 类型，业务单元。
- `volume`: `i32` 类型，成交量。

### 示例代码

```rust
async fn simulate_market_data(api: &mut CThostFtdcTraderApi) {
    let mut interval = time::interval(Duration::from_millis(500));
    let mut rng = StdRng::from_entropy();

    loop {
        interval.tick().await;
        let market_quote = FakeMarketQuote {
            instrument_id: "ag2406".to_string(),
            bid_price: rng.gen_range(1000.0..2000.0),
            ask_price: rng.gen_range(1000.0..2000.0),
            quote_ref: format!("{:.1}", rng.gen::<f64>() * 1000.0),
            last_price: format!("{:.2}", rng.gen::<f64>() * 5000.0),
            settlement_price: format!("{:.2}", rng.gen::<f64>() * 5000.0),
            upper_limit_price: format!("{:.2}", rng.gen::<f64>() * 6000.0),
            lower_limit_price: format!("{:.2}", rng.gen::<f64>() * 4000.0),
            business_unit: format!("{:08}", rng.gen::<u32>()),
            volume: rng.gen_range(1..100),
        };

        if let Err(e) = api.insert_market_quote(market_quote) {
            eprintln!("Error inserting market quote: {:?}", e);
        }
    }
}
```

### 注意事项

- 定时器的间隔可以根据实际需求进行调整。
- 随机数生成器使用的是 `StdRng::from_entropy()`，确保线程安全并能在多线程环境中使用。
- 插入市场数据时需要处理可能的错误情况，并进行适当的错误处理。
