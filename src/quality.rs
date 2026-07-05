#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualityFlag {
    Good,
    Estimated,
    Manual,
    Stale,
    Clamped,
    SensorFault,
    RadioLoss,
    Maintenance,
    VendorSpecific,
}

pub fn decode_quality(raw: u8) -> QualityFlag {
    match raw & 0x0f {
        0 => QualityFlag::Good,
        1 => QualityFlag::Estimated,
        2 => QualityFlag::Manual,
        3 => QualityFlag::Stale,
        4 => QualityFlag::Clamped,
        5 => QualityFlag::SensorFault,
        6 => QualityFlag::RadioLoss,
        7 => QualityFlag::Maintenance,
        _ => QualityFlag::VendorSpecific,
    }
}

pub fn quality_score(raw: u8) -> u16 {
    match decode_quality(raw) {
        QualityFlag::Good => 0,
        QualityFlag::Estimated => 4,
        QualityFlag::Manual => 6,
        QualityFlag::Stale => 16,
        QualityFlag::Clamped => 12,
        QualityFlag::SensorFault => 35,
        QualityFlag::RadioLoss => 25,
        QualityFlag::Maintenance => 8,
        QualityFlag::VendorSpecific => 10,
    }
}

pub fn quality_name(raw: u8) -> &'static str {
    match decode_quality(raw) {
        QualityFlag::Good => "good",
        QualityFlag::Estimated => "estimated",
        QualityFlag::Manual => "manual",
        QualityFlag::Stale => "stale",
        QualityFlag::Clamped => "clamped",
        QualityFlag::SensorFault => "sensor-fault",
        QualityFlag::RadioLoss => "radio-loss",
        QualityFlag::Maintenance => "maintenance",
        QualityFlag::VendorSpecific => "vendor-specific",
    }
}
