# Security Policy

## Reporting a Vulnerability

Report security vulnerabilities to **amu.hlongwane@swelabs.io**.

Please include:

- A description of the vulnerability and its potential impact.
- Steps to reproduce or a proof-of-concept.
- Affected versions.

We aim to acknowledge reports within 48 hours and provide a remediation
timeline within 7 business days.

## Scope

This crate contains no network I/O, no credential handling, and no secrets.
The load balancer config stores only backend URLs and weights — never tokens,
passwords, or keys.
