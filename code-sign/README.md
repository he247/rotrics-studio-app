# Code-sign assets policy

This folder may contain packaging/signing helpers for Windows installers.

## Recommended handling
- Do **not** commit private keys (`*.pfx`, `*.p12`, raw key files) to Git.
- Keep signing certificates and passwords in your secret manager / CI secrets.
- Keep only non-sensitive tooling or public instructions in this directory.

## Current state
- `code-sign/win/note.text` is a translated operational note.
- Any binary signing tools should be periodically reviewed for provenance and checksum verification before use.
