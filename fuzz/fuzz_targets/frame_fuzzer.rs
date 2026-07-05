#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = sluiceledger::parse_frames(data);
});
