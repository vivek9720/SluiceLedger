#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineeringUnit {
    Millimeter,
    MillimeterPerHour,
    Ampere,
    Millivolt,
    LiterPerSecond,
    Percent,
    Celsius,
    Raw,
}

impl EngineeringUnit {
    pub fn from_code(code: u8) -> Self {
        match code {
            1 => Self::Millimeter,
            2 => Self::MillimeterPerHour,
            3 => Self::Ampere,
            4 => Self::Millivolt,
            5 => Self::LiterPerSecond,
            6 => Self::Percent,
            7 => Self::Celsius,
            _ => Self::Raw,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Millimeter => "mm",
            Self::MillimeterPerHour => "mm/h",
            Self::Ampere => "A",
            Self::Millivolt => "mV",
            Self::LiterPerSecond => "L/s",
            Self::Percent => "%",
            Self::Celsius => "C",
            Self::Raw => "raw",
        }
    }
}

pub fn scale_raw(metric: &str, raw: i32) -> f32 {
    if metric.contains("rain") {
        raw as f32 / 10.0
    } else if metric.contains("pump") {
        raw as f32 / 25.0
    } else if metric.contains("flow") {
        raw as f32 / 5.0
    } else {
        raw as f32
    }
}

pub fn display_value(metric: &str, raw: i32, unit_code: u8) -> String {
    let unit = EngineeringUnit::from_code(unit_code);
    format!("{:.2} {}", scale_raw(metric, raw), unit.symbol())
}
