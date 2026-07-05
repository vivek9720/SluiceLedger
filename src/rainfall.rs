#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RainBand {
    Dry,
    Light,
    Moderate,
    Heavy,
    Cloudburst,
}

pub fn classify_mm_h(raw_tenths: i32) -> RainBand {
    match raw_tenths {
        i32::MIN..=0 => RainBand::Dry,
        1..=60 => RainBand::Light,
        61..=180 => RainBand::Moderate,
        181..=500 => RainBand::Heavy,
        _ => RainBand::Cloudburst,
    }
}

pub fn runoff_multiplier(raw_tenths: i32, impervious_percent: u8) -> f32 {
    let rain = (raw_tenths.max(0) as f32) / 10.0;
    let pavement = impervious_percent.min(100) as f32 / 100.0;
    1.0 + rain / 50.0 + pavement * 0.75
}

pub fn band_name(band: RainBand) -> &'static str {
    match band {
        RainBand::Dry => "dry",
        RainBand::Light => "light",
        RainBand::Moderate => "moderate",
        RainBand::Heavy => "heavy",
        RainBand::Cloudburst => "cloudburst",
    }
}
