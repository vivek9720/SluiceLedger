use crate::ledger;
use crate::statistics;
use crate::topology_tools;
use crate::StormReport;

pub fn render_text(report: &StormReport) -> String {
    let digest = ledger::digest(report);
    let levels = statistics::reading_stats(report, "level");
    let topology = topology_tools::summarize(report);
    let mut out = String::new();
    out.push_str("SluiceLedger report\n");
    out.push_str(&format!("stations={}\n", digest.station_count));
    out.push_str(&format!("readings={}\n", digest.reading_count));
    out.push_str(&format!("alarms={}\n", digest.alarm_count));
    out.push_str(&format!("maintenance={}\n", digest.maintenance_count));
    out.push_str(&format!("retired_pages={}\n", digest.retired_pages));
    out.push_str(&format!("level_min={} level_max={} level_avg={:.2}\n", levels.min, levels.max, levels.average));
    out.push_str(&format!("topology_nodes={} topology_edges={}\n", topology.nodes, topology.edges));
    out.push_str(&format!("digest={:08x}\n", digest.digest));
    out
}

pub fn render_alarm_lines(report: &StormReport) -> Vec<String> {
    report
        .alarms
        .iter()
        .enumerate()
        .map(|(idx, alarm)| format!("{idx:04}:{alarm}"))
        .collect()
}
