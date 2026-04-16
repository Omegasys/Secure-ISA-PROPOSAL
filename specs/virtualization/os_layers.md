# Operating System Layers

## Overview

OS layers provide system services within a controlled virtualization context.

## Levels

* O0: Primary OS (host OS)
* O1–O3: Guest OS instances
* O4: Minimal runtime OS

## Responsibilities

* Process scheduling
* Memory management (within domain constraints)
* System call handling

## Constraints

* OS cannot bypass capability system
* Memory management must respect hardware-enforced bounds
* No direct hardware access without hypervisor mediation

## Capability Interaction

OS is responsible for:

* Distributing capabilities to programs
* Enforcing policy constraints

## Isolation

* Each OS operates in a separate domain
* Cross-OS interaction requires hypervisor mediation

## Security Guarantees

* OS compromise is contained within its domain
* Cannot escalate to hypervisor without explicit capability

## Use Cases

* Guest operating systems
* Container hosts
* Secure runtime environments
