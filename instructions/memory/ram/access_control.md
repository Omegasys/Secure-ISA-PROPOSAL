# Physical Memory Access Control

## Overview

Defines how physical memory access is restricted.

## Enforcement

* Capability-based access required
* Permission bits enforced:

  * READ
  * WRITE
  * EXECUTE

## Domain Binding

* Memory pages belong to a domain
* Cross-domain access requires explicit capability

## Hardware Checks

On each access:

1. Capability validation
2. Bounds check
3. Permission check
4. Domain check

## Violations

* Trigger memory fault
* No partial execution allowed

## Security Guarantees

* Prevents unauthorized access
* Enforces strict isolation
