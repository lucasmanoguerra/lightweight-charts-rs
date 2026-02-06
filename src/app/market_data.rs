use crate::chart::{sample_candles, Candle, ChartStyle, HistogramPoint, LinePoint};
use crate::indicators::rsi::compute_rsi;
use serde_json::Value;
use time::OffsetDateTime;

pub struct MarketData {
    pub candles: Vec<Candle>,
    pub volumes: Vec<HistogramPoint>,
    pub rsi: Vec<LinePoint>,
    pub symbol: String,
    pub interval: String,
}

pub struct MarketBatch {
    pub candles: Vec<Candle>,
    pub volumes: Vec<HistogramPoint>,
}

pub struct MarketStore {
    pub candles: Vec<Candle>,
    pub volumes: Vec<HistogramPoint>,
    pub rsi: Vec<LinePoint>,
    pub symbol: String,
    pub interval: String,
    pub interval_ms: i64,
    pub earliest_ms: i64,
    pub latest_ms: i64,
}

pub struct KlineEvent {
    pub open_time_ms: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub is_final: bool,
}

pub struct KlineApplyResult {
    pub candle: Candle,
    pub volume: HistogramPoint,
}

const BINANCE_SYMBOL: &str = "BTCUSDT";
const BINANCE_INTERVAL: &str = "1m";
const BINANCE_LIMIT: usize = 500;

impl MarketStore {
    pub fn new(
        candles: Vec<Candle>,
        volumes: Vec<HistogramPoint>,
        rsi: Vec<LinePoint>,
        symbol: String,
        interval: String,
    ) -> Self {
        let interval_ms = interval_to_millis(&interval).unwrap_or(60_000);
        let earliest_ms = candles.first().map(|c| time_to_ms(c.time)).unwrap_or(0);
        let latest_ms = candles.last().map(|c| time_to_ms(c.time)).unwrap_or(0);
        Self {
            candles,
            volumes,
            rsi,
            symbol,
            interval,
            interval_ms,
            earliest_ms,
            latest_ms,
        }
    }

    pub fn prepend_batch(&mut self, batch: MarketBatch) -> bool {
        if batch.candles.is_empty() {
            return false;
        }
        self.candles.extend(batch.candles);
        self.candles.sort_by(|a, b| a.time.cmp(&b.time));
        self.candles.dedup_by(|a, b| a.time == b.time);

        self.volumes.extend(batch.volumes);
        self.volumes.sort_by(|a, b| a.time.cmp(&b.time));
        self.volumes.dedup_by(|a, b| a.time == b.time);

        self.earliest_ms = self
            .candles
            .first()
            .map(|c| time_to_ms(c.time))
            .unwrap_or(self.earliest_ms);
        self.latest_ms = self
            .candles
            .last()
            .map(|c| time_to_ms(c.time))
            .unwrap_or(self.latest_ms);

        self.rsi = compute_rsi(&self.candles, 14);
        true
    }

    pub fn apply_kline(&mut self, event: KlineEvent, style: &ChartStyle) -> KlineApplyResult {
        let time = time_from_ms(event.open_time_ms).unwrap_or_else(|_| {
            OffsetDateTime::from_unix_timestamp(0).unwrap()
        });
        let candle = Candle {
            time,
            open: event.open,
            high: event.high,
            low: event.low,
            close: event.close,
        };
        let color = if candle.close >= candle.open {
            style.up
        } else {
            style.down
        };
        let volume = HistogramPoint {
            time,
            value: event.volume,
            color: Some(color),
        };

        update_sorted_candles(&mut self.candles, candle.clone());
        update_sorted_volumes(&mut self.volumes, volume.clone());

        self.earliest_ms = self
            .candles
            .first()
            .map(|c| time_to_ms(c.time))
            .unwrap_or(self.earliest_ms);
        self.latest_ms = self
            .candles
            .last()
            .map(|c| time_to_ms(c.time))
            .unwrap_or(self.latest_ms);

        self.rsi = compute_rsi(&self.candles, 14);

        KlineApplyResult { candle, volume }
    }
}

pub fn load_market_data() -> MarketData {
    match fetch_binance_klines(BINANCE_SYMBOL, BINANCE_INTERVAL, BINANCE_LIMIT) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Binance fetch failed: {err}. Falling back to sample data.");
            sample_data()
        }
    }
}

pub fn fetch_binance_klines(
    symbol: &str,
    interval: &str,
    limit: usize,
) -> Result<MarketData, String> {
    let urls = [
        build_klines_url("https://api.binance.com", symbol, interval, limit, None, None),
        build_klines_url("https://data-api.binance.vision", symbol, interval, limit, None, None),
    ];

    let mut last_err = None;
    for url in urls {
        match fetch_klines_from_url(&url) {
            Ok(rows) => return parse_klines(rows, symbol, interval),
            Err(err) => last_err = Some(err),
        }
    }

    Err(last_err.unwrap_or_else(|| "Unable to fetch Binance klines".to_string()))
}

pub fn fetch_binance_klines_batch(
    symbol: &str,
    interval: &str,
    limit: usize,
    start_time_ms: Option<i64>,
    end_time_ms: Option<i64>,
) -> Result<MarketBatch, String> {
    let urls = [
        build_klines_url(
            "https://api.binance.com",
            symbol,
            interval,
            limit,
            start_time_ms,
            end_time_ms,
        ),
        build_klines_url(
            "https://data-api.binance.vision",
            symbol,
            interval,
            limit,
            start_time_ms,
            end_time_ms,
        ),
    ];

    let mut last_err = None;
    for url in urls {
        match fetch_klines_from_url(&url) {
            Ok(rows) => {
                let (candles, volumes) = parse_klines_rows(rows)?;
                return Ok(MarketBatch { candles, volumes });
            }
            Err(err) => last_err = Some(err),
        }
    }

    Err(last_err.unwrap_or_else(|| "Unable to fetch Binance klines".to_string()))
}

fn sample_data() -> MarketData {
    let candles = sample_candles();
    let style = ChartStyle::default();
    let volumes = candles
        .iter()
        .map(|candle| {
            let volume = (candle.high - candle.low).abs();
            let color = if candle.close >= candle.open {
                style.up
            } else {
                style.down
            };
            HistogramPoint {
                time: candle.time,
                value: volume,
                color: Some(color),
            }
        })
        .collect();
    let rsi = compute_rsi(&candles, 14);
    MarketData {
        candles,
        volumes,
        rsi,
        symbol: BINANCE_SYMBOL.to_string(),
        interval: BINANCE_INTERVAL.to_string(),
    }
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

fn build_klines_url(
    base: &str,
    symbol: &str,
    interval: &str,
    limit: usize,
    start_time_ms: Option<i64>,
    end_time_ms: Option<i64>,
) -> String {
    let mut url = format!("{base}/api/v3/klines?symbol={symbol}&interval={interval}&limit={limit}");
    if let Some(start) = start_time_ms {
        url.push_str(&format!("&startTime={start}"));
    }
    if let Some(end) = end_time_ms {
        url.push_str(&format!("&endTime={end}"));
    }
    url
}

fn fetch_klines_from_url(url: &str) -> Result<Vec<Value>, String> {
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    let value: Value = response.into_json().map_err(|e| e.to_string())?;
    value
        .as_array()
        .cloned()
        .ok_or_else(|| "Invalid Binance response: expected array".to_string())
}

fn parse_klines_rows(rows: Vec<Value>) -> Result<(Vec<Candle>, Vec<HistogramPoint>), String> {
    let style = ChartStyle::default();
    let mut candles = Vec::with_capacity(rows.len());
    let mut volumes = Vec::with_capacity(rows.len());

    for row in rows {
        let fields = row
            .as_array()
            .ok_or_else(|| "Invalid Binance row: expected array".to_string())?;
        if fields.len() < 6 {
            return Err("Invalid Binance row: not enough fields".to_string());
        }

        let open_time = parse_i64(&fields[0]).ok_or_else(|| "Invalid open time".to_string())?;
        let open = parse_f64(&fields[1]).ok_or_else(|| "Invalid open".to_string())?;
        let high = parse_f64(&fields[2]).ok_or_else(|| "Invalid high".to_string())?;
        let low = parse_f64(&fields[3]).ok_or_else(|| "Invalid low".to_string())?;
        let close = parse_f64(&fields[4]).ok_or_else(|| "Invalid close".to_string())?;
        let volume = parse_f64(&fields[5]).ok_or_else(|| "Invalid volume".to_string())?;

        let time = time_from_ms(open_time)?;
        let candle = Candle {
            time,
            open,
            high,
            low,
            close,
        };
        let volume_color = if close >= open { style.up } else { style.down };

        candles.push(candle);
        volumes.push(HistogramPoint {
            time,
            value: volume,
            color: Some(volume_color),
        });
    }

    Ok((candles, volumes))
}

fn parse_klines(rows: Vec<Value>, symbol: &str, interval: &str) -> Result<MarketData, String> {
    let (candles, volumes) = parse_klines_rows(rows)?;
    let rsi = compute_rsi(&candles, 14);
    Ok(MarketData {
        candles,
        volumes,
        rsi,
        symbol: symbol.to_string(),
        interval: interval.to_string(),
    })
}

fn time_from_ms(ms: i64) -> Result<OffsetDateTime, String> {
    let secs = ms / 1000;
    let nanos = (ms % 1000) * 1_000_000;
    let base =
        OffsetDateTime::from_unix_timestamp(secs).map_err(|e| format!("{e:?}"))?;
    Ok(base + time::Duration::nanoseconds(nanos as i64))
}

fn time_to_ms(time: OffsetDateTime) -> i64 {
    time.unix_timestamp() * 1000 + (time.nanosecond() as i64 / 1_000_000)
}

fn interval_to_millis(interval: &str) -> Option<i64> {
    if interval.len() < 2 {
        return None;
    }
    let (num_part, unit_part) = interval.split_at(interval.len() - 1);
    let value: i64 = num_part.parse().ok()?;
    let multiplier = match unit_part {
        "m" => 60_000,
        "h" => 60_000 * 60,
        "d" => 60_000 * 60 * 24,
        "w" => 60_000 * 60 * 24 * 7,
        "M" => 60_000 * 60 * 24 * 30,
        _ => return None,
    };
    Some(value * multiplier)
}

fn update_sorted_candles(data: &mut Vec<Candle>, item: Candle) {
    match data.last_mut() {
        Some(last) if last.time == item.time => {
            *last = item;
        }
        Some(last) if last.time < item.time => {
            data.push(item);
        }
        _ => {
            data.push(item);
            data.sort_by(|a, b| a.time.cmp(&b.time));
        }
    }
}

fn update_sorted_volumes(data: &mut Vec<HistogramPoint>, item: HistogramPoint) {
    match data.last_mut() {
        Some(last) if last.time == item.time => {
            *last = item;
        }
        Some(last) if last.time < item.time => {
            data.push(item);
        }
        _ => {
            data.push(item);
            data.sort_by(|a, b| a.time.cmp(&b.time));
        }
    }
}
