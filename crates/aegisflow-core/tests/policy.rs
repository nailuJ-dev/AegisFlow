<<<<<<< HEAD
<<<<<<< HEAD
#![allow(clippy::unwrap_used)]

=======
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
#![allow(clippy::unwrap_used)]

>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
use aegisflow_core::{Capability, DataLabel, Operation, PolicyEngine, ToolRequest};

#[test]
fn secret_data_cannot_be_sent_to_network() {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
    let request = ToolRequest::new(
        "workflow-1",
        Operation::NetworkPost,
        DataLabel::Secret,
        "payload",
    );
<<<<<<< HEAD
    let decision = PolicyEngine::default().evaluate(&request, &[]);
    assert!(!decision.allowed);
    assert_eq!(
        decision.reason,
        "secret data cannot cross the network boundary"
    );
=======
    let request = ToolRequest::new(Operation::NetworkPost, DataLabel::Secret, "payload");
=======
    let request = ToolRequest::new("workflow-1", Operation::NetworkPost, DataLabel::Secret, "payload");
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
    let decision = PolicyEngine::default().evaluate(&request, &[]);
    assert!(!decision.allowed);
    assert_eq!(decision.reason, "secret data cannot cross the network boundary");
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
    let decision = PolicyEngine::default().evaluate(&request, &[]);
    assert!(!decision.allowed);
    assert_eq!(
        decision.reason,
        "secret data cannot cross the network boundary"
    );
>>>>>>> 9910f1a (Push first version)
}

#[test]
fn valid_capability_authorizes_file_read() {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
    let request = ToolRequest::new(
        "workflow-1",
        Operation::FileRead,
        DataLabel::Trusted,
        "/safe/input.txt",
    );
<<<<<<< HEAD
=======
    let request = ToolRequest::new(Operation::FileRead, DataLabel::Trusted, "/safe/input.txt");
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
    let request = ToolRequest::new("workflow-1", Operation::FileRead, DataLabel::Trusted, "/safe/input.txt");
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
=======
>>>>>>> 9910f1a (Push first version)
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
<<<<<<< HEAD
<<<<<<< HEAD

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
=======
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======

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
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
