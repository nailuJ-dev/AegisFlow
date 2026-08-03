//! Deny-by-default policy, capability, taint-label, and tamper-evident audit primitives.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

const MAX_ARGUMENT_BYTES: usize = 64 * 1024;

/// Classification attached to data as it crosses workflow boundaries.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DataLabel {
    /// Public information.
    Public,
    /// Data produced by a trusted source.
    Trusted,
    /// Data controlled by an external or untrusted source.
    Untrusted,
    /// Confidential data that must not cross network boundaries.
    Secret,
}

/// Operations that require explicit policy decisions.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    /// Read a local file through a constrained adapter.
    FileRead,
    /// Write a local file through a constrained adapter.
    FileWrite,
    /// Retrieve a network resource.
    NetworkGet,
    /// Send data to a network resource.
    NetworkPost,
    /// Read a secret from an approved secret store.
    SecretRead,
}

/// A typed request produced by a planner. It is not executed directly.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolRequest {
    /// Stable workflow or principal identifier.
    pub subject: String,
    /// Requested operation.
    pub operation: Operation,
    /// Taint label of the argument.
    pub label: DataLabel,
    /// Bounded non-secret argument or resource identifier.
    pub argument: String,
}

impl ToolRequest {
    /// Creates a request. Oversized arguments are retained but denied by policy.
    #[must_use]
    pub fn new(
        subject: impl Into<String>,
        operation: Operation,
        label: DataLabel,
        argument: impl Into<String>,
    ) -> Self {
        Self { subject: subject.into(), operation, label, argument: argument.into() }
    }
}

/// Short-lived authorization scoped to an operation and workflow subject.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Capability {
    id: Uuid,
    operation: Operation,
    subject: String,
    expires_unix_seconds: u64,
}

/// Capability construction errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CapabilityError {
    /// Subject was empty or unreasonably large.
    #[error("capability subject must contain 1..=256 bytes")]
    InvalidSubject,
    /// TTL was zero or exceeded the supported maximum.
    #[error("capability TTL must contain 1..=3600 seconds")]
    InvalidTtl,
    /// System clock could not be represented.
    #[error("system clock is before the Unix epoch")]
    Clock,
}

impl Capability {
    /// Issues a bounded capability. Production deployments should replace this local issuer with a signed authority.
    pub fn issue(operation: Operation, subject: impl Into<String>, ttl_seconds: u64) -> Result<Self, CapabilityError> {
        let subject = subject.into();
        if subject.is_empty() || subject.len() > 256 { return Err(CapabilityError::InvalidSubject); }
        if !(1..=3600).contains(&ttl_seconds) { return Err(CapabilityError::InvalidTtl); }
        let now = unix_seconds()?;
        Ok(Self { id: Uuid::new_v4(), operation, subject, expires_unix_seconds: now.saturating_add(ttl_seconds) })
    }

    fn authorizes(&self, operation: Operation, subject: &str, now: u64) -> bool {
        self.operation == operation && self.subject == subject && now <= self.expires_unix_seconds
    }
}

/// Explainable policy result.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Decision {
    /// Whether execution may proceed.
    pub allowed: bool,
    /// Stable human-readable reason.
    pub reason: &'static str,
}

/// Stateless deny-by-default reference policy.
#[derive(Clone, Debug, Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    /// Evaluates label flow, request bounds, and capabilities.
    #[must_use]
    pub fn evaluate(&self, request: &ToolRequest, capabilities: &[Capability]) -> Decision {
        if request.subject.is_empty() || request.subject.len() > 256 {
            return Decision { allowed: false, reason: "workflow subject is outside configured bounds" };
        }
        if request.argument.len() > MAX_ARGUMENT_BYTES {
            return Decision { allowed: false, reason: "argument exceeds the configured policy limit" };
        }
        if request.argument.as_bytes().contains(&0) {
            return Decision { allowed: false, reason: "argument contains a NUL byte" };
        }
        if request.label == DataLabel::Secret && matches!(request.operation, Operation::NetworkGet | Operation::NetworkPost) {
            return Decision { allowed: false, reason: "secret data cannot cross the network boundary" };
        }
        if request.label == DataLabel::Untrusted && matches!(request.operation, Operation::FileWrite | Operation::SecretRead) {
            return Decision { allowed: false, reason: "untrusted data cannot drive a sensitive operation" };
        }
        let now = unix_seconds().unwrap_or(u64::MAX);
        if capabilities.iter().any(|capability| capability.authorizes(request.operation, &request.subject, now)) {
            Decision { allowed: true, reason: "matching unexpired capability" }
        } else {
            Decision { allowed: false, reason: "no matching unexpired capability" }
        }
    }
}

/// A tamper-evident audit entry.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct AuditEntry {
    sequence: u64,
    workflow_id: String,
    event: String,
    previous_hash: String,
    hash: String,
}

/// Append-only in-memory audit chain. Persist entries through an external append-only adapter.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AuditChain { entries: Vec<AuditEntry> }

/// Audit validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuditError {
    /// Workflow or event text exceeded a safe bound.
    #[error("audit fields exceed configured bounds")]
    Oversized,
}

impl AuditChain {
    /// Appends a hashed event.
    pub fn append(&mut self, workflow_id: impl Into<String>, event: impl Into<String>) -> Result<&AuditEntry, AuditError> {
        let workflow_id = workflow_id.into();
        let event = event.into();
        if workflow_id.is_empty() || workflow_id.len() > 256 || event.is_empty() || event.len() > 4096 {
            return Err(AuditError::Oversized);
        }
        let sequence = u64::try_from(self.entries.len()).unwrap_or(u64::MAX);
        let previous_hash = self.entries.last().map_or_else(|| "0".repeat(64), |entry| entry.hash.clone());
        let hash = calculate_hash(sequence, &workflow_id, &event, &previous_hash);
        self.entries.push(AuditEntry { sequence, workflow_id, event, previous_hash, hash });
        self.entries.last().ok_or(AuditError::Oversized)
    }

    /// Verifies sequence and hash linkage.
    #[must_use]
    pub fn verify(&self) -> bool {
        let mut previous = "0".repeat(64);
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.sequence != u64::try_from(index).unwrap_or(u64::MAX) || entry.previous_hash != previous {
                return false;
            }
            let expected = calculate_hash(entry.sequence, &entry.workflow_id, &entry.event, &entry.previous_hash);
            if entry.hash != expected { return false; }
            previous.clone_from(&entry.hash);
        }
        true
    }

    /// Entries for persistence or inspection.
    #[must_use]
    pub fn entries(&self) -> &[AuditEntry] { &self.entries }
}

fn calculate_hash(sequence: u64, workflow_id: &str, event: &str, previous_hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sequence.to_be_bytes());
    hasher.update((workflow_id.len() as u64).to_be_bytes());
    hasher.update(workflow_id.as_bytes());
    hasher.update((event.len() as u64).to_be_bytes());
    hasher.update(event.as_bytes());
    hasher.update(previous_hash.as_bytes());
    hex::encode(hasher.finalize())
}

fn unix_seconds() -> Result<u64, CapabilityError> {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_secs()).map_err(|_| CapabilityError::Clock)
}
