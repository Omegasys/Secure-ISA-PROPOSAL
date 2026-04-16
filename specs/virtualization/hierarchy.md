# Virtualization Hierarchy

## Overview

The ISA supports structured, nested virtualization through a controlled hierarchy of execution layers.

## Design Goals

* Strong isolation between layers
* Deterministic transitions
* Minimal trusted boundaries
* Avoid combinatorial complexity

## Layer Model

Instead of fixed arbitrary levels, the hierarchy is structured as:

1. Hypervisor Layer (L0–L4)
2. OS Layer (O0–O4)
3. Program Layer (P0–P5)

Each layer is:

* Isolated via capabilities
* Identified by a Layer ID (LID)
* Enforced by hardware

## Execution Context

Each running context includes:

* Hypervisor Level (HL)
* OS Level (OL)
* Program Level (PL)
* Domain ID
* Capability Set

## Layer Composition Rule

Execution context is defined as:

Context = {HL, OL, PL, Domain, Capabilities}

## Transition Rules

Transitions between layers require:

* Explicit instructions
* Capability validation
* Isolation boundary checks

## Constraints

* Lower layers cannot directly access higher layers
* All cross-layer interaction must be mediated
* No implicit privilege escalation

## Security Guarantees

* Strict vertical isolation
* Controlled delegation
* Verifiable execution paths
