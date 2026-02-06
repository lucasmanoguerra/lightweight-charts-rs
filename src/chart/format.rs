use time::OffsetDateTime;

use super::types::{PriceFormat, PriceScaleMode, TimeLabelMode};

pub(crate) fn format_time_label(
    time: f64,
    step: i64,
    mode: TimeLabelMode,
    custom: &str,
    time_visible: bool,
    seconds_visible: bool,
    tick_mark_format: &str,
    tick_mark_max_len: usize,
) -> String {
    let timestamp = time.round() as i64;
    let dt = OffsetDateTime::from_unix_timestamp(timestamp).unwrap_or(OffsetDateTime::UNIX_EPOCH);

    let mut label = if !tick_mark_format.trim().is_empty() {
        format_time_custom(tick_mark_format, dt)
    } else {
        match mode {
            TimeLabelMode::Time => {
                let t = dt.time();
                if seconds_visible {
                    format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second())
                } else {
                    format!("{:02}:{:02}", t.hour(), t.minute())
                }
            }
            TimeLabelMode::Date => {
                let d = dt.date();
                format!("{:04}-{:02}-{:02}", d.year(), u8::from(d.month()), d.day())
            }
            TimeLabelMode::DateTime => {
                let d = dt.date();
                let t = dt.time();
                if seconds_visible {
                    format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                        d.year(),
                        u8::from(d.month()),
                        d.day(),
                        t.hour(),
                        t.minute(),
                        t.second()
                    )
                } else {
                    format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}",
                        d.year(),
                        u8::from(d.month()),
                        d.day(),
                        t.hour(),
                        t.minute()
                    )
                }
            }
            TimeLabelMode::Custom => {
                if custom.trim().is_empty() {
                    format_time_label(
                        time,
                        step,
                        TimeLabelMode::Auto,
                        custom,
                        time_visible,
                        seconds_visible,
                        tick_mark_format,
                        tick_mark_max_len,
                    )
                } else {
                    format_time_custom(custom, dt)
                }
            }
            TimeLabelMode::Auto => {
                if !time_visible {
                    if step < 30 * 24 * 60 * 60 {
                        let d = dt.date();
                        format!("{:02}-{:02}", u8::from(d.month()), d.day())
                    } else if step < 365 * 24 * 60 * 60 {
                        let d = dt.date();
                        format!("{:04}-{:02}", d.year(), u8::from(d.month()))
                    } else {
                        format!("{:04}", dt.date().year())
                    }
                } else if step < 60 && seconds_visible {
                    let t = dt.time();
                    format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second())
                } else if step < 24 * 60 * 60 {
                    let t = dt.time();
                    format!("{:02}:{:02}", t.hour(), t.minute())
                } else if step < 30 * 24 * 60 * 60 {
                    let d = dt.date();
                    format!("{:02}-{:02}", u8::from(d.month()), d.day())
                } else {
                    let d = dt.date();
                    format!("{:04}-{:02}", d.year(), u8::from(d.month()))
                }
            }
        }
    };

    if tick_mark_max_len > 0 && label.len() > tick_mark_max_len {
        label.truncate(tick_mark_max_len);
    }

    label
}

fn format_time_custom(format: &str, dt: OffsetDateTime) -> String {
    let d = dt.date();
    let t = dt.time();
    let mut text = format.to_string();
    text = text.replace("{YYYY}", &format!("{:04}", d.year()));
    text = text.replace("{YY}", &format!("{:02}", d.year() % 100));
    text = text.replace("{MM}", &format!("{:02}", u8::from(d.month())));
    text = text.replace("{DD}", &format!("{:02}", d.day()));
    text = text.replace("{HH}", &format!("{:02}", t.hour()));
    text = text.replace("{mm}", &format!("{:02}", t.minute()));
    text = text.replace("{ss}", &format!("{:02}", t.second()));
    text
}

pub(crate) fn format_price(value: f64, precision: usize) -> String {
    format!("{value:.precision$}", precision = precision)
}

pub(crate) fn format_price_with_format(
    value: f64,
    format: &PriceFormat,
    fallback_precision: usize,
    mode: PriceScaleMode,
) -> String {
    let (value, precision, suffix) = match (mode, format) {
        (PriceScaleMode::Percentage, _) | (PriceScaleMode::IndexedTo100, _) => {
            let precision = match format {
                PriceFormat::Percent { precision } => *precision,
                PriceFormat::Price { precision, .. } => *precision,
                PriceFormat::Volume { precision } => *precision,
            };
            (value, precision, "%")
        }
        (_, PriceFormat::Percent { precision }) => (value, *precision, "%"),
        (_, PriceFormat::Volume { precision }) => (value, *precision, ""),
        (_, PriceFormat::Price { precision, .. }) => (value, *precision, ""),
    };
    let precision = precision.max(fallback_precision);
    let text = format!("{value:.precision$}", precision = precision);
    if suffix.is_empty() {
        text
    } else {
        format!("{text}{suffix}")
    }
}

pub(crate) fn format_tooltip(
    template: &str,
    candle: &super::types::Candle,
    precision: usize,
    format: &PriceFormat,
    mode: PriceScaleMode,
) -> String {
    let mut text = template.to_string();
    text = text.replace("{time}", &format_datetime(candle.time));
    text = text.replace(
        "{open}",
        &format_price_with_format(candle.open, format, precision, mode),
    );
    text = text.replace(
        "{high}",
        &format_price_with_format(candle.high, format, precision, mode),
    );
    text = text.replace(
        "{low}",
        &format_price_with_format(candle.low, format, precision, mode),
    );
    text = text.replace(
        "{close}",
        &format_price_with_format(candle.close, format, precision, mode),
    );
    text
}

pub(crate) fn format_series_tooltip(
    template: &str,
    series: &str,
    time: OffsetDateTime,
    value: f64,
    precision: usize,
    format: &PriceFormat,
    mode: PriceScaleMode,
) -> String {
    let mut text = template.to_string();
    text = text.replace("{series}", series);
    text = text.replace("{time}", &format_datetime(time));
    text = text.replace(
        "{value}",
        &format_price_with_format(value, format, precision, mode),
    );
    text
}

pub(crate) fn format_datetime(dt: OffsetDateTime) -> String {
    let d = dt.date();
    let t = dt.time();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        d.year(),
        u8::from(d.month()),
        d.day(),
        t.hour(),
        t.minute()
    )
}
