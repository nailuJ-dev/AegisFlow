# ADR 0001: No executor in the initial public service

Status: Accepted.

The service evaluates proposed actions but cannot access files, network, or secrets. Real executors must be separate least-privilege processes and require a new threat-model review.
