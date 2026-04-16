# Isolation Model

## Overview

Isolation is enforced across all virtualization layers using a unified hardware model.

## Isolation Dimensions

### 1. Memory Isolation

* Enforced via capabilities
* No raw addressing
* Tagged memory ensures validity

### 2. Execution Isolation

* Separate execution contexts
* No shared registers across domains

### 3. Domain Isolation

* Each entity operates within a domain
* Domains cannot interfere without permission

### 4. Layer Isolation

* Strict separation between hypervisor, OS, and program layers

## Enforcement Mechanisms

* Capability validation on every access
* Domain ID checks
* Layer boundary checks

## Communication Model

Allowed via:

* Message passing
* Shared read-only memory
* Explicit capability transfer

## Prohibited Actions

* Direct cross-domain memory access
* Implicit privilege escalation
* Unauthorized capability creation

## Fault Containment

* Faults are contained within domain
* No propagation across layers

## Security Guarantees

* Strong multi-tenant isolation
* Resistance to privilege escalation
* Deterministic boundary enforcement

## Tradeoffs

* Increased overhead for communication
* More complex scheduling

## Design Principle

Isolation is not optional—it is the default state of execution.
