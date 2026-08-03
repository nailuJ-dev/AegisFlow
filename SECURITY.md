# Security policy

## Supported versions

Security fixes are applied to the latest minor release on the `main` branch.

## Reporting

Do not open public issues for suspected vulnerabilities. Use GitHub private vulnerability
reporting for `nailuJ-dev/aegisflow`. Include the affected version, reproduction steps, impact,
and any proposed mitigation. Do not include real credentials or sensitive production data.

## Security boundaries

- Inputs are untrusted at every API, CLI, file, and model boundary.
- Dynamic code loading and executable model formats are not supported.
- Secrets must be injected at runtime and must never be committed.
- Releases should be signed and accompanied by an SBOM and provenance attestation.
