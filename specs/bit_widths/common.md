# Common Bit-Width Architecture

## Overview

All bit-width variants (32, 64, 128, 256) share a unified ISA core with consistent semantics. Differences are limited to:

* Register width
* Address space size
* Capability size
* Performance characteristics

## Design Goals

* Cross-width compatibility
* Minimal divergence in instruction behavior
* Scalable security guarantees

## Shared Features

* Capability-based memory access
* Tagged memory model
* Deterministic execution support
* Constant-time execution mode (optional)

## Compatibility Rules

### Forward Compatibility

* Lower bit-width binaries MAY execute on higher bit-width systems
* Zero/sign extension rules must be deterministic

### Backward Compatibility

* Higher bit-width instructions MUST NOT execute on lower bit-width systems

## Instruction Consistency

All instructions:

* Maintain identical opcodes across widths
* Differ only in operand size and encoding extensions

## Capability Scaling

Capabilities scale with bit-width:

* Address fields expand
* Bounds precision increases
* Metadata remains consistent

## Security Guarantees

Security properties MUST remain invariant across all widths:

* No raw memory access
* Mandatory capability enforcement
* Domain isolation preserved
