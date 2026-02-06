pub(crate) struct TimeTicks {
    pub(crate) ticks: Vec<f64>,
    pub(crate) step: i64,
}

pub(crate) struct PriceTicks {
    pub(crate) ticks: Vec<f64>,
    pub(crate) precision: usize,
}

pub(crate) fn build_time_ticks(
    start: f64,
    end: f64,
    plot_width: f64,
    uniform_distribution: bool,
) -> TimeTicks {
    let range = (end - start).max(1.0);
    let target_ticks = (plot_width / 110.0).clamp(3.0, 10.0);
    let step = if uniform_distribution {
        (range / target_ticks).max(1.0).round() as i64
    } else {
        choose_tick_step(range, target_ticks)
    };
    let mut ticks = Vec::new();
    if uniform_distribution {
        let mut current = start;
        while current <= end {
            ticks.push(current);
            current += step as f64;
        }
    } else {
        let mut first = (start as i64 / step) * step;
        if (first as f64) < start {
            first += step;
        }
        let mut current = first;
        while (current as f64) <= end {
            ticks.push(current as f64);
            current += step;
        }
    }

    if ticks.is_empty() {
        ticks.push(start);
        ticks.push(end);
    }

    TimeTicks { ticks, step }
}

fn choose_tick_step(range: f64, target_ticks: f64) -> i64 {
    let candidates = [
        1,
        5,
        10,
        15,
        30,
        60,
        5 * 60,
        15 * 60,
        30 * 60,
        60 * 60,
        2 * 60 * 60,
        4 * 60 * 60,
        6 * 60 * 60,
        12 * 60 * 60,
        24 * 60 * 60,
        2 * 24 * 60 * 60,
        7 * 24 * 60 * 60,
        30 * 24 * 60 * 60,
        90 * 24 * 60 * 60,
        365 * 24 * 60 * 60,
    ];

    for step in candidates {
        if range / step as f64 <= target_ticks {
            return step;
        }
    }

    *candidates.last().unwrap_or(&86400)
}

pub(crate) fn build_price_ticks(min: f64, max: f64, plot_height: f64) -> PriceTicks {
    let range = (max - min).max(1.0);
    let target_ticks = (plot_height / 60.0).clamp(4.0, 8.0);
    let step = nice_step(range / (target_ticks - 1.0));
    let nice_min = (min / step).floor() * step;
    let nice_max = (max / step).ceil() * step;
    let mut ticks = Vec::new();
    let mut value = nice_min;
    while value <= nice_max + (step * 0.5) {
        ticks.push(value);
        value += step;
    }

    let precision = price_precision(step);
    PriceTicks { ticks, precision }
}

fn nice_step(raw_step: f64) -> f64 {
    let exponent = raw_step.log10().floor();
    let base = 10_f64.powf(exponent);
    let fraction = raw_step / base;
    let nice_fraction = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 2.5 {
        2.5
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * base
}

fn price_precision(step: f64) -> usize {
    if step >= 1.0 {
        0
    } else {
        let digits = (-step.log10()).ceil() as i32;
        digits.clamp(0, 6) as usize
    }
}
