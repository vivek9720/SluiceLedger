use std::collections::BTreeMap;

use crate::codec_tables;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalibrationCurve {
    pub offset: i16,
    pub scale_milli: i16,
}

#[derive(Clone, Debug, Default)]
pub struct CalibrationBook {
    curves: BTreeMap<(u16, String), CalibrationCurve>,
}

impl CalibrationBook {
    pub fn insert(&mut self, station_id: u16, metric: impl Into<String>, offset: i16, scale_milli: i16) {
        self.curves.insert((station_id, metric.into()), CalibrationCurve { offset, scale_milli });
    }

    pub fn apply(&self, station_id: u16, metric: &str, raw: i32) -> f32 {
        if let Some(curve) = self.curves.get(&(station_id, metric.to_string())) {
            (raw + curve.offset as i32) as f32 * curve.scale_milli as f32 / 1000.0
        } else if codec_tables::metric_by_name(metric).is_some() {
            crate::units::scale_raw(metric, raw)
        } else {
            raw as f32
        }
    }

    pub fn len(&self) -> usize {
        self.curves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.curves.is_empty()
    }
}
