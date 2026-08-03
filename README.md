# AegisFlow

A deny-by-default workflow policy runtime for AI agents. Planners can propose typed tool requests,
but only the policy engine can authorize them. Secret-to-network and untrusted-to-sensitive flows
are structurally denied, and sensitive operations require short-lived capabilities.

```bash
cargo test --workspace
cargo run -p aegisflow-cli -- \
  --operation file-read --label trusted --argument /safe/input.txt --issue-capability
cargo run -p aegisflow-service
```

The included capability issuer is local and intentionally minimal. Replace it with a signed,
identity-bound authority before multi-tenant deployment. No actual filesystem, network, or secret
executor is enabled in this scaffold.
