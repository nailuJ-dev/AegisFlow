//! Deny-by-default policy, capability, taint-label, and tamper-evident audit primitives.

<<<<<<< HEAD
<<<<<<< HEAD
use std::time::{SystemTime, UNIX_EPOCH};
=======
use std::time::{Duration, SystemTime, UNIX_EPOCH};
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
use std::time::{SystemTime, UNIX_EPOCH};
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

const MAX_ARGUMENT_BYTES: usize = 64 * 1024;

/// Classification attached to data as it crosses workflow boundaries.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
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
<<<<<<< HEAD
=======
pub enum DataLabel { Public, Trusted, Untrusted, Secret }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)

/// Operations that require explicit policy decisions.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
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
<<<<<<< HEAD
=======
pub enum Operation { FileRead, FileWrite, NetworkGet, NetworkPost, SecretRead }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)

/// A typed request produced by a planner. It is not executed directly.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolRequest {
<<<<<<< HEAD
<<<<<<< HEAD
    /// Stable workflow or principal identifier.
    pub subject: String,
=======
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
    /// Stable workflow or principal identifier.
    pub subject: String,
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
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
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
    pub fn new(
        subject: impl Into<String>,
        operation: Operation,
        label: DataLabel,
        argument: impl Into<String>,
    ) -> Self {
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
        Self {
            subject: subject.into(),
            operation,
            label,
            argument: argument.into(),
        }
<<<<<<< HEAD
=======
    pub fn new(operation: Operation, label: DataLabel, argument: impl Into<String>) -> Self {
        Self { operation, label, argument: argument.into() }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
        Self { subject: subject.into(), operation, label, argument: argument.into() }
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
=======
>>>>>>> 9910f1a (Push first version)
    }
}

/// Short-lived authorization scoped to an operation and workflow subject.
<<<<<<< HEAD
<<<<<<< HEAD
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
=======
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
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
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
    pub fn issue(
        operation: Operation,
        subject: impl Into<String>,
        ttl_seconds: u64,
    ) -> Result<Self, CapabilityError> {
<<<<<<< HEAD
        let subject = subject.into();
        if subject.is_empty() || subject.len() > 256 {
            return Err(CapabilityError::InvalidSubject);
        }
        if !(1..=3600).contains(&ttl_seconds) {
            return Err(CapabilityError::InvalidTtl);
        }
        let now = unix_seconds()?;
        Ok(Self {
            id: Uuid::new_v4(),
            operation,
            subject,
            expires_unix_seconds: now.saturating_add(ttl_seconds),
        })
    }

    fn authorizes(&self, operation: Operation, subject: &str, now: u64) -> bool {
        self.operation == operation && self.subject == subject && now <= self.expires_unix_seconds
=======
    pub fn issue(operation: Operation, subject: impl Into<String>, ttl_seconds: u64) -> Result<Self, CapabilityError> {
=======
>>>>>>> 9910f1a (Push first version)
        let subject = subject.into();
        if subject.is_empty() || subject.len() > 256 {
            return Err(CapabilityError::InvalidSubject);
        }
        if !(1..=3600).contains(&ttl_seconds) {
            return Err(CapabilityError::InvalidTtl);
        }
        let now = unix_seconds()?;
        Ok(Self {
            id: Uuid::new_v4(),
            operation,
            subject,
            expires_unix_seconds: now.saturating_add(ttl_seconds),
        })
    }

<<<<<<< HEAD
    fn authorizes(&self, operation: Operation, now: u64) -> bool {
        self.operation == operation && now <= self.expires_unix_seconds
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
    fn authorizes(&self, operation: Operation, subject: &str, now: u64) -> bool {
        self.operation == operation && self.subject == subject && now <= self.expires_unix_seconds
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
    }
}

/// Explainable policy result.
<<<<<<< HEAD
<<<<<<< HEAD
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
=======
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
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
<<<<<<< HEAD
<<<<<<< HEAD
        if request.subject.is_empty() || request.subject.len() > 256 {
            return Decision {
                allowed: false,
                reason: "workflow subject is outside configured bounds",
            };
        }
        if request.argument.len() > MAX_ARGUMENT_BYTES {
            return Decision {
                allowed: false,
                reason: "argument exceeds the configured policy limit",
            };
        }
        if request.argument.as_bytes().contains(&0) {
            return Decision {
                allowed: false,
                reason: "argument contains a NUL byte",
            };
        }
        if request.label == DataLabel::Secret
            && matches!(
                request.operation,
                Operation::NetworkGet | Operation::NetworkPost
            )
        {
            return Decision {
                allowed: false,
                reason: "secret data cannot cross the network boundary",
            };
        }
        if request.label == DataLabel::Untrusted
            && matches!(
                request.operation,
                Operation::FileWrite | Operation::SecretRead
            )
        {
            return Decision {
                allowed: false,
                reason: "untrusted data cannot drive a sensitive operation",
            };
        }
        let now = unix_seconds().unwrap_or(u64::MAX);
        if capabilities
            .iter()
            .any(|capability| capability.authorizes(request.operation, &request.subject, now))
        {
            Decision {
                allowed: true,
                reason: "matching unexpired capability",
            }
        } else {
            Decision {
                allowed: false,
                reason: "no matching unexpired capability",
            }
=======
=======
        if request.subject.is_empty() || request.subject.len() > 256 {
            return Decision {
                allowed: false,
                reason: "workflow subject is outside configured bounds",
            };
        }
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
        if request.argument.len() > MAX_ARGUMENT_BYTES {
            return Decision {
                allowed: false,
                reason: "argument exceeds the configured policy limit",
            };
        }
        if request.argument.as_bytes().contains(&0) {
            return Decision {
                allowed: false,
                reason: "argument contains a NUL byte",
            };
        }
        if request.label == DataLabel::Secret
            && matches!(
                request.operation,
                Operation::NetworkGet | Operation::NetworkPost
            )
        {
            return Decision {
                allowed: false,
                reason: "secret data cannot cross the network boundary",
            };
        }
        if request.label == DataLabel::Untrusted
            && matches!(
                request.operation,
                Operation::FileWrite | Operation::SecretRead
            )
        {
            return Decision {
                allowed: false,
                reason: "untrusted data cannot drive a sensitive operation",
            };
        }
        let now = unix_seconds().unwrap_or(u64::MAX);
        if capabilities
            .iter()
            .any(|capability| capability.authorizes(request.operation, &request.subject, now))
        {
            Decision {
                allowed: true,
                reason: "matching unexpired capability",
            }
        } else {
<<<<<<< HEAD
            Decision { allowed: false, reason: "no matching unexpired capability" }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
            Decision {
                allowed: false,
                reason: "no matching unexpired capability",
            }
>>>>>>> 9910f1a (Push first version)
        }
    }
}

/// A tamper-evident audit entry.
<<<<<<< HEAD
<<<<<<< HEAD
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
=======
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
pub struct AuditEntry {
    sequence: u64,
    workflow_id: String,
    event: String,
    previous_hash: String,
    hash: String,
}

/// Append-only in-memory audit chain. Persist entries through an external append-only adapter.
<<<<<<< HEAD
<<<<<<< HEAD
#[derive(Clone, Debug, Default, Serialize)]
pub struct AuditChain {
    entries: Vec<AuditEntry>,
}
<<<<<<< HEAD
=======
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
=======
#[derive(Clone, Debug, Default, Serialize)]
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
pub struct AuditChain { entries: Vec<AuditEntry> }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
>>>>>>> 9910f1a (Push first version)

/// Audit validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuditError {
    /// Workflow or event text exceeded a safe bound.
    #[error("audit fields exceed configured bounds")]
    Oversized,
}

impl AuditChain {
    /// Appends a hashed event.
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
    pub fn append(
        &mut self,
        workflow_id: impl Into<String>,
        event: impl Into<String>,
    ) -> Result<&AuditEntry, AuditError> {
<<<<<<< HEAD
        let workflow_id = workflow_id.into();
        let event = event.into();
        if workflow_id.is_empty()
            || workflow_id.len() > 256
            || event.is_empty()
            || event.len() > 4096
        {
            return Err(AuditError::Oversized);
        }
        let sequence = u64::try_from(self.entries.len()).unwrap_or(u64::MAX);
        let previous_hash = self
            .entries
            .last()
            .map_or_else(|| "0".repeat(64), |entry| entry.hash.clone());
        let hash = calculate_hash(sequence, &workflow_id, &event, &previous_hash);
        self.entries.push(AuditEntry {
            sequence,
            workflow_id,
            event,
            previous_hash,
            hash,
        });
=======
    pub fn append(&mut self, workflow_id: impl Into<String>, event: impl Into<String>) -> Result<&AuditEntry, AuditError> {
=======
>>>>>>> 9910f1a (Push first version)
        let workflow_id = workflow_id.into();
        let event = event.into();
        if workflow_id.is_empty()
            || workflow_id.len() > 256
            || event.is_empty()
            || event.len() > 4096
        {
            return Err(AuditError::Oversized);
        }
        let sequence = u64::try_from(self.entries.len()).unwrap_or(u64::MAX);
        let previous_hash = self
            .entries
            .last()
            .map_or_else(|| "0".repeat(64), |entry| entry.hash.clone());
        let hash = calculate_hash(sequence, &workflow_id, &event, &previous_hash);
<<<<<<< HEAD
        self.entries.push(AuditEntry { sequence, workflow_id, event, previous_hash, hash });
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
        self.entries.push(AuditEntry {
            sequence,
            workflow_id,
            event,
            previous_hash,
            hash,
        });
>>>>>>> 9910f1a (Push first version)
        self.entries.last().ok_or(AuditError::Oversized)
    }

    /// Verifies sequence and hash linkage.
    #[must_use]
    pub fn verify(&self) -> bool {
        let mut previous = "0".repeat(64);
        for (index, entry) in self.entries.iter().enumerate() {
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
            if entry.sequence != u64::try_from(index).unwrap_or(u64::MAX)
                || entry.previous_hash != previous
            {
                return false;
            }
            let expected = calculate_hash(
                entry.sequence,
                &entry.workflow_id,
                &entry.event,
                &entry.previous_hash,
            );
            if entry.hash != expected {
<<<<<<< HEAD
                return false;
            }
=======
            if entry.sequence != u64::try_from(index).unwrap_or(u64::MAX) || entry.previous_hash != previous {
                return false;
            }
            let expected = calculate_hash(entry.sequence, &entry.workflow_id, &entry.event, &entry.previous_hash);
            if entry.hash != expected { return false; }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
                return false;
            }
>>>>>>> 9910f1a (Push first version)
            previous.clone_from(&entry.hash);
        }
        true
    }

    /// Entries for persistence or inspection.
    #[must_use]
<<<<<<< HEAD
<<<<<<< HEAD
    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }
=======
    pub fn entries(&self) -> &[AuditEntry] { &self.entries }
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }
>>>>>>> 9910f1a (Push first version)
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
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9910f1a (Push first version)
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| CapabilityError::Clock)
<<<<<<< HEAD
=======
    SystemTime::now().duration_since(UNIX_EPOCH).map(Duration::as_secs).map_err(|_| CapabilityError::Clock)
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
    SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_secs()).map_err(|_| CapabilityError::Clock)
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
=======
>>>>>>> 9910f1a (Push first version)
}
