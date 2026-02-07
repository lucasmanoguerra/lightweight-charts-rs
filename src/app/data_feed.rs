use serde_json::Value;
use std::sync::mpsc::Sender;
use std::{thread, time};
use tungstenite::{connect, Message};
use url::Url;

use super::market_data::{fetch_binance_klines_batch, KlineEvent, MarketBatch, MarketStore};

pub enum DataEvent {
    Prepend(MarketBatch),
    Kline(KlineEvent),
    LoadFailed(String),
}

pub struct LazyLoader {
    batch_size: usize,
    threshold_bars: i64,
    loading: bool,
    exhausted: bool,
}

impl LazyLoader {
    pub fn new(batch_size: usize, threshold_bars: i64) -> Self {
        Self {
            batch_size,
            threshold_bars,
            loading: false,
            exhausted: false,
        }
    }

    pub fn maybe_request(
        &mut self,
        chart: &crate::chart::ChartApi,
        store: &MarketStore,
        sender: &Sender<DataEvent>,
    ) {
        if self.loading || self.exhausted {
            return;
        }

        let (start_time, _) = chart.visible_time_range();
        let start_ms = (start_time * 1000.0) as i64;
        let threshold_ms = store.interval_ms.saturating_mul(self.threshold_bars);
        if start_ms - store.earliest_ms > threshold_ms {
            return;
        }

        let symbol = store.symbol.clone();
        let interval = store.interval.clone();
        let end_time_ms = store.earliest_ms.saturating_sub(1);
        let batch_size = self.batch_size;
        let sender = sender.clone();
        self.loading = true;

        thread::spawn(move || {
            match fetch_binance_klines_batch(
                &symbol,
                &interval,
                batch_size,
                None,
                Some(end_time_ms),
            ) {
                Ok(batch) => {
                    let _ = sender.send(DataEvent::Prepend(batch));
                }
                Err(err) => {
                    let _ = sender.send(DataEvent::LoadFailed(err));
                }
            }
        });
    }

    pub fn finish_success(&mut self, loaded_any: bool) {
        self.loading = false;
        if !loaded_any {
            self.exhausted = true;
        }
    }

    pub fn finish_failure(&mut self) {
        self.loading = false;
    }
}

pub fn spawn_kline_stream(symbol: String, interval: String, sender: Sender<DataEvent>) {
    thread::spawn(move || {
        let stream = format!("{}@kline_{}", symbol.to_lowercase(), interval);
        let url = format!("wss://stream.binance.com:9443/ws/{}", stream);
        let Ok(url) = Url::parse(&url) else {
            eprintln!("Invalid WebSocket URL: {url}");
            return;
        };
        loop {
            let connect_result = connect(url.as_str());
            match connect_result {
                Ok((mut socket, _)) => loop {
                    match socket.read() {
                        Ok(Message::Text(text)) => {
                            if let Some(event) = parse_kline_message(&text) {
                                let _ = sender.send(DataEvent::Kline(event));
                            }
                        }
                        Ok(Message::Ping(payload)) => {
                            let _ = socket.send(Message::Pong(payload));
                        }
                        Ok(Message::Close(_)) => break,
                        Ok(_) => {}
                        Err(_) => break,
                    }
                },
                Err(_) => {}
            }
            thread::sleep(time::Duration::from_secs(2));
        }
    });
}

fn parse_kline_message(text: &str) -> Option<KlineEvent> {
    let value: Value = serde_json::from_str(text).ok()?;
    if value.get("e").and_then(|v| v.as_str()) != Some("kline") {
        return None;
    }
    let kline = value.get("k")?;

    let open_time_ms = parse_i64(kline.get("t")?)?;
    let open = parse_f64(kline.get("o")?)?;
    let high = parse_f64(kline.get("h")?)?;
    let low = parse_f64(kline.get("l")?)?;
    let close = parse_f64(kline.get("c")?)?;
    let volume = parse_f64(kline.get("v")?)?;
    let is_final = kline.get("x").and_then(|v| v.as_bool()).unwrap_or(false);

    Some(KlineEvent {
        open_time_ms,
        open,
        high,
        low,
        close,
        volume,
        is_final,
    })
}

fn parse_f64(value: &Value) -> Option<f64> {
    if let Some(value) = value.as_f64() {
        return Some(value);
    }
    value.as_str()?.parse::<f64>().ok()
}

fn parse_i64(value: &Value) -> Option<i64> {
    if let Some(value) = value.as_i64() {
        return Some(value);
    }
    value.as_str()?.parse::<i64>().ok()
}
