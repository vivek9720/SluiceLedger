use crate::wire;
use crate::{parse_frames, Frame};

#[derive(Clone, Debug, Default)]
pub struct ReplayPlan {
    pub frames: usize,
    pub dictionaries: usize,
    pub event_batches: usize,
    pub controls: usize,
    pub unknown: usize,
}

pub fn plan_from_frames(frames: &[Frame]) -> ReplayPlan {
    let mut plan = ReplayPlan::default();
    plan.frames = frames.len();
    for frame in frames {
        match wire::classify(frame.kind) {
            wire::WireClass::Dictionary => plan.dictionaries += 1,
            wire::WireClass::Event | wire::WireClass::Replay => plan.event_batches += 1,
            wire::WireClass::Control => plan.controls += 1,
            wire::WireClass::Unknown => plan.unknown += 1,
            _ => {}
        }
    }
    plan
}

pub fn plan_from_stream(data: &[u8]) -> ReplayPlan {
    parse_frames(data)
        .map(|frames| plan_from_frames(&frames))
        .unwrap_or_default()
}
