# Hypervisor Layers

## Overview

Hypervisor layers manage virtualized hardware and system-level isolation.

## Levels

* L0: Root Hypervisor (hardware control)
* L1–L3: Nested hypervisors
* L4: Lightweight virtualization layer

## Responsibilities

* Virtual CPU management
* Memory virtualization
* Device isolation
* Domain creation

## Capabilities

Hypervisors control:

* Domain allocation
* Capability delegation
* Interrupt routing

## Isolation Model

Each hypervisor:

* Operates in its own domain
* Cannot access peer hypervisors without explicit capability

## Nested Virtualization

Supported with constraints:

* Each level must enforce isolation below it
* Resource limits must be enforced hierarchically

## Security Constraints

* L0 is minimal trusted computing base
* Higher levels cannot override lower-level protections
* Capability checks enforced at every boundary

## Failure Containment

* Compromise at Ln does not affect L(n-1)
* Fault isolation is mandatory

## Use Cases

* Cloud infrastructure
* Secure multi-tenant systems
* Sandbox orchestration
