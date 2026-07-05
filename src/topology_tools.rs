use std::collections::{BTreeMap, BTreeSet};

use crate::StormReport;

#[derive(Clone, Debug, Default)]
pub struct TopologyStats {
    pub nodes: usize,
    pub edges: usize,
    pub duplicate_edges: usize,
    pub terminal_nodes: usize,
}

pub fn summarize(report: &StormReport) -> TopologyStats {
    let mut nodes = BTreeMap::<u16, usize>::new();
    let mut seen = BTreeSet::<(u16, u16)>::new();
    let mut duplicate_edges = 0;
    for (from, to, _) in &report.edges {
        *nodes.entry(*from).or_default() += 1;
        *nodes.entry(*to).or_default() += 1;
        let key = if from <= to { (*from, *to) } else { (*to, *from) };
        if !seen.insert(key) {
            duplicate_edges += 1;
        }
    }
    TopologyStats {
        nodes: nodes.len(),
        edges: report.edges.len(),
        duplicate_edges,
        terminal_nodes: nodes.values().filter(|count| **count <= 1).count(),
    }
}

pub fn path_label(from: u16, to: u16, label: &str) -> String {
    format!("{from:04x}->{to:04x}:{label}")
}
