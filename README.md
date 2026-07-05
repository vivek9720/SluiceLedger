# SluiceLedger

SluiceLedger is a Rust decoder for municipal stormwater pump-station captures. Field gateways record compact binary streams during storms: dictionary pages, radio frames, fragmented TLV batches, journal records, topology edges, and maintenance directives. The project is intentionally self-contained for offline fuzzing: no registry dependencies, four cargo-fuzz style harnesses, ClusterFuzzLite metadata, and seed corpora in detected locations.

The full path is multi-stage: `SLDG` envelope parsing, optional payload expansion, scoped dictionary installation, fragment reassembly, TLV dispatch, journal replay, topology/risk evaluation, and report finalization. Stateful dictionary values are represented as leases into page storage so lifecycle mistakes are observable under AddressSanitizer when the parser replays realistic maintenance traffic.
