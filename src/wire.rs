use crate::FrameKind;

pub const STREAM_MAGIC: &[u8; 4] = b"SLDG";
pub const JOURNAL_MAGIC: &[u8; 4] = b"JNL1";
pub const FLAG_COMPRESSED: u8 = 0x01;
pub const FLAG_RELAXED_CHECKSUM: u8 = 0x80;
pub const MAX_STREAM_FRAMES: usize = 96;
pub const MAX_FRAME_PAYLOAD: usize = 65_535;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireClass {
    Session,
    Dictionary,
    Replay,
    Event,
    Asset,
    Control,
    Unknown,
}

pub fn classify(kind: FrameKind) -> WireClass {
    match kind {
        FrameKind::Hello => WireClass::Session,
        FrameKind::Dictionary => WireClass::Dictionary,
        FrameKind::Fragment | FrameKind::Journal => WireClass::Replay,
        FrameKind::Batch | FrameKind::Snapshot => WireClass::Event,
        FrameKind::Topology | FrameKind::Calibration => WireClass::Asset,
        FrameKind::Maintenance => WireClass::Control,
        FrameKind::Unknown(_) => WireClass::Unknown,
    }
}

pub fn frame_name(kind: FrameKind) -> &'static str {
    match kind {
        FrameKind::Hello => "hello",
        FrameKind::Dictionary => "dictionary",
        FrameKind::Fragment => "fragment",
        FrameKind::Batch => "batch",
        FrameKind::Journal => "journal",
        FrameKind::Topology => "topology",
        FrameKind::Calibration => "calibration",
        FrameKind::Maintenance => "maintenance",
        FrameKind::Snapshot => "snapshot",
        FrameKind::Unknown(_) => "unknown",
    }
}

pub fn flags_summary(flags: u8) -> String {
    let mut parts = Vec::new();
    if flags & FLAG_COMPRESSED != 0 {
        parts.push("compressed");
    }
    if flags & FLAG_RELAXED_CHECKSUM != 0 {
        parts.push("relaxed-checksum");
    }
    if flags & 0x02 != 0 {
        parts.push("priority");
    }
    if flags & 0x04 != 0 {
        parts.push("radio-repeat");
    }
    if parts.is_empty() {
        "plain".to_string()
    } else {
        parts.join(",")
    }
}

pub fn plausible_frame_count(value: u8) -> bool {
    value as usize <= MAX_STREAM_FRAMES
}
