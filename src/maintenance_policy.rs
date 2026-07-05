use crate::MaintenanceOp;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaintenanceDecision {
    Ignore,
    Audit,
    RetireDictionary,
    Escalate,
}

pub fn classify(op: &MaintenanceOp) -> MaintenanceDecision {
    match (op.action, op.mode & 0x03, op.retired_pages) {
        (3, _, pages) if pages > 0 => MaintenanceDecision::RetireDictionary,
        (3, 1, 0) => MaintenanceDecision::Audit,
        (7, _, _) => MaintenanceDecision::Escalate,
        _ => MaintenanceDecision::Ignore,
    }
}

pub fn maintenance_note(op: &MaintenanceOp) -> String {
    let decision = classify(op);
    format!(
        "scope={} action={} generation={} decision={:?} retired={}",
        op.scope, op.action, op.generation, decision, op.retired_pages
    )
}

pub fn is_stateful_action(action: u8) -> bool {
    matches!(action, 3 | 4 | 7 | 11)
}
