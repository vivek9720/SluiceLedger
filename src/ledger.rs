use crate::maintenance_policy;
use crate::StormReport;

#[derive(Clone, Debug, Default)]
pub struct LedgerDigest {
    pub station_count: usize,
    pub reading_count: usize,
    pub alarm_count: usize,
    pub maintenance_count: usize,
    pub retired_pages: usize,
    pub digest: u32,
}

pub fn digest(report: &StormReport) -> LedgerDigest {
    let mut state = 0x51d6_011du32;
    for (station_id, name) in &report.stations {
        state ^= (*station_id as u32).rotate_left(5);
        for byte in name.as_bytes() {
            state = state.rotate_left(3) ^ *byte as u32;
        }
    }
    let retired_pages = report.maintenance.iter().map(|op| op.retired_pages).sum();
    for op in &report.maintenance {
        state ^= (op.token ^ op.generation as u32).rotate_left((op.scope & 15) as u32);
        if maintenance_policy::is_stateful_action(op.action) {
            state = state.wrapping_mul(0x45d9_f3b);
        }
    }
    LedgerDigest {
        station_count: report.stations.len(),
        reading_count: report.readings.len(),
        alarm_count: report.alarms.len(),
        maintenance_count: report.maintenance.len(),
        retired_pages,
        digest: state,
    }
}
