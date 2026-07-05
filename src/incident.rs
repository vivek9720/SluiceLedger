use crate::rainfall::{self, RainBand};
use crate::{profiles, StormReport};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncidentHint {
    pub station_id: u16,
    pub severity: u16,
    pub reason: String,
}

pub fn derive_incidents(report: &StormReport) -> Vec<IncidentHint> {
    let mut hints = Vec::new();
    for (station_id, metric, raw) in &report.readings {
        let mut severity = 0u16;
        if let Some(profile) = profiles::lookup(*station_id) {
            if metric.contains("level") && *raw > profile.flood_floor_mm as i32 {
                severity += 60;
            }
            if metric.contains("battery") && *raw < profile.battery_floor_mv as i32 {
                severity += 25;
            }
        }
        if metric.contains("rain") && rainfall::classify_mm_h(*raw) >= RainBand::Heavy {
            severity += 30;
        }
        if severity > 0 {
            hints.push(IncidentHint {
                station_id: *station_id,
                severity,
                reason: format!("{metric}:{raw}"),
            });
        }
    }
    hints.sort_by_key(|hint| std::cmp::Reverse(hint.severity));
    hints
}
