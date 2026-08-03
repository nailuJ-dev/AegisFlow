# AegisFlow Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Deliver an explainable capability-secured policy runtime.

**Architecture:** Pure policy primitives live in `aegisflow-core`; CLI and HTTP adapters never execute proposed tools.

**Tech Stack:** Rust 2024, serde, SHA-256, UUID, axum, clap.

## Global Constraints
- Deny by default.
- No direct LLM-to-tool execution.
- No secrets in logs or API errors.

### Task 1: Policy primitives
- [x] Write tests for secret exfiltration, capabilities, and audit integrity.
- [x] Implement labels, operations, expiring capabilities, policy decisions, and hash chain.

### Task 2: Adapters and operations
- [x] Add CLI and bounded HTTP evaluator.
- [x] Add hardened deployment and supply-chain automation.
