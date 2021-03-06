const PRODUCTION_RATE_DEFAULT: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed as u32 {
        0 => 0.0,
        speed @ 1..=4 => (PRODUCTION_RATE_DEFAULT * speed) as f64,
        speed @ 5..=8 => (PRODUCTION_RATE_DEFAULT * speed) as f64 * 0.9,
        speed @ 9..=10 => (PRODUCTION_RATE_DEFAULT * speed) as f64 * 0.77,
        _ => panic!("`speed` should be in [0, 10]."),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
