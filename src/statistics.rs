use crate::StormReport;

#[derive(Clone, Debug, Default)]
pub struct ReadingStats {
    pub count: usize,
    pub min: i32,
    pub max: i32,
    pub average: f32,
}

pub fn reading_stats(report: &StormReport, metric_filter: &str) -> ReadingStats {
    let mut stats = ReadingStats {
        min: i32::MAX,
        max: i32::MIN,
        ..ReadingStats::default()
    };
    let mut total = 0i64;
    for (_, metric, raw) in &report.readings {
        if metric.contains(metric_filter) {
            stats.count += 1;
            stats.min = stats.min.min(*raw);
            stats.max = stats.max.max(*raw);
            total += *raw as i64;
        }
    }
    if stats.count == 0 {
        stats.min = 0;
        stats.max = 0;
    } else {
        stats.average = total as f32 / stats.count as f32;
    }
    stats
}

pub fn warning_density(report: &StormReport) -> f32 {
    let events = report.readings.len() + report.alarms.len() + report.maintenance.len();
    if events == 0 {
        0.0
    } else {
        report.warnings.len() as f32 / events as f32
    }
}
