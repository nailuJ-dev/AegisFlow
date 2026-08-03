# Threat model

## Assets

Correctness of aegisflow, availability of its service, integrity of inputs and outputs, audit
evidence, and confidentiality of runtime configuration.

## Trust boundaries

1. HTTP and CLI inputs are attacker-controlled.
2. Files, model artifacts, manifests, and benchmark samples are untrusted.
3. Container and orchestration configuration are operator-controlled.
4. Scientific or statistical outputs are advisory and require domain review.

## Primary threats and mitigations

- Resource exhaustion: body limits, bounded vectors, explicit size validation, and timeouts.
- Parser abuse: serde-based schemas, denial of unknown fields where appropriate, and fuzzing.
- Supply-chain compromise: locked dependencies in release builds, cargo-deny, cargo-audit,
  Dependabot, SBOM generation, and signed provenance.
- Information leakage: structured errors, no secret values in logs, and least-privilege containers.
- Integrity failure: deterministic reference implementations, property tests, and explicit hashes.

## Out of scope

Certification for safety-critical, cryptographic, aerospace, medical, or defense deployment.
Production adoption requires independent review, validation against authoritative datasets,
and operational controls appropriate to the deployment environment.
