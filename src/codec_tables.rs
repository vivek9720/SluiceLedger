use crate::units::EngineeringUnit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricSpec {
    pub dictionary_key: &'static str,
    pub canonical: &'static str,
    pub unit: EngineeringUnit,
    pub soft_min: i32,
    pub soft_max: i32,
    pub dispatch_weight: u16,
}

pub const METRICS: &[MetricSpec] = &[
    MetricSpec { dictionary_key: "metric.level", canonical: "level_mm", unit: EngineeringUnit::Millimeter, soft_min: 0, soft_max: 1800, dispatch_weight: 40 },
    MetricSpec { dictionary_key: "metric.rain", canonical: "rain_mm_h", unit: EngineeringUnit::MillimeterPerHour, soft_min: 0, soft_max: 800, dispatch_weight: 20 },
    MetricSpec { dictionary_key: "metric.pump", canonical: "pump_amp", unit: EngineeringUnit::Ampere, soft_min: 0, soft_max: 1800, dispatch_weight: 25 },
    MetricSpec { dictionary_key: "metric.gate", canonical: "gate_mm", unit: EngineeringUnit::Millimeter, soft_min: 0, soft_max: 2400, dispatch_weight: 15 },
    MetricSpec { dictionary_key: "metric.battery", canonical: "battery_mv", unit: EngineeringUnit::Millivolt, soft_min: 11_800, soft_max: 14_800, dispatch_weight: 30 },
    MetricSpec { dictionary_key: "metric.flow", canonical: "flow_lps", unit: EngineeringUnit::LiterPerSecond, soft_min: 0, soft_max: 3200, dispatch_weight: 35 },
    MetricSpec { dictionary_key: "metric.radio", canonical: "radio_retry", unit: EngineeringUnit::Raw, soft_min: 0, soft_max: 8, dispatch_weight: 12 },
    MetricSpec { dictionary_key: "metric.temp", canonical: "cabinet_c", unit: EngineeringUnit::Celsius, soft_min: -20, soft_max: 70, dispatch_weight: 8 },
];

pub fn metric_by_key(key: &str) -> Option<MetricSpec> {
    METRICS.iter().copied().find(|metric| metric.dictionary_key == key)
}

pub fn metric_by_name(name: &str) -> Option<MetricSpec> {
    METRICS.iter().copied().find(|metric| metric.canonical == name)
}

pub fn metric_weight(name: &str, raw: i32) -> u16 {
    let Some(metric) = metric_by_name(name) else {
        return 0;
    };
    if raw < metric.soft_min || raw > metric.soft_max {
        metric.dispatch_weight
    } else {
        0
    }
}
