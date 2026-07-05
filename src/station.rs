use crate::profiles;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StationKey {
    pub district: u8,
    pub station_id: u16,
}

impl StationKey {
    pub fn new(station_id: u16) -> Self {
        let district = profiles::lookup(station_id)
            .map(|profile| profile.district)
            .unwrap_or(0);
        Self { district, station_id }
    }

    pub fn shard(self) -> u16 {
        ((self.district as u16) << 10) ^ (self.station_id & 0x03ff)
    }
}

pub fn station_label(station_id: u16) -> String {
    if let Some(profile) = profiles::lookup(station_id) {
        format!("{}:{}:{:04x}", profile.basin, profile.pump_family, station_id)
    } else {
        format!("unmapped:{station_id:04x}")
    }
}

pub fn station_capacity_hint(station_id: u16) -> u16 {
    profiles::lookup(station_id)
        .map(|profile| profile.max_gate_mm / 10 + profile.radio_class as u16)
        .unwrap_or(64)
}
