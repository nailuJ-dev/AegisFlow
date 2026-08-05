#![allow(clippy::unwrap_used)]

use aegisflow_core::{Capability, DataLabel, Operation, PolicyEngine, ToolRequest};

#[test]
fn secret_data_cannot_be_sent_to_network() {
    let request = ToolRequest::new(
        "workflow-1",
        Operation::NetworkPost,
        DataLabel::Secret,
        "payload",
    );
    let decision = PolicyEngine::default().evaluate(&request, &[]);
    assert!(!decision.allowed);
    assert_eq!(
        decision.reason,
        "secret data cannot cross the network boundary"
    );
}

#[test]
fn valid_capability_authorizes_file_read() {
    let request = ToolRequest::new(
        "workflow-1",
        Operation::FileRead,
        DataLabel::Trusted,
        "/safe/input.txt",
    );
    let capability = Capability::issue(Operation::FileRead, "workflow-1", 60).unwrap();
    let decision = PolicyEngine::default().evaluate(&request, &[capability]);
    assert!(decision.allowed);
}

#[test]
fn audit_chain_detects_mutation() {
    let mut chain = aegisflow_core::AuditChain::default();
    chain.append("workflow-1", "allow").unwrap();
    chain.append("workflow-1", "deny").unwrap();
    assert!(chain.verify());
}

#[test]
fn capability_for_another_subject_is_rejected() {
    let request = ToolRequest::new(
        "workflow-2",
        Operation::FileRead,
        DataLabel::Trusted,
        "/safe/input.txt",
    );
    let capability = Capability::issue(Operation::FileRead, "workflow-1", 60).unwrap();
    assert!(
        !PolicyEngine::default()
            .evaluate(&request, &[capability])
            .allowed
    );
}
