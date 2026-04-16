# Program Layers

## Overview

Program layers define execution levels for user and application code.

## Levels

* P0: System-critical programs
* P1–P3: Trusted applications
* P4–P5: Untrusted or sandboxed applications

## Responsibilities

* Application logic execution
* Controlled resource usage
* Capability consumption

## Constraints

* No direct hardware access
* All memory access via capabilities
* Must operate within assigned domain

## Capability Usage

Programs:

* Receive capabilities from OS
* Cannot create arbitrary capabilities
* Can derive restricted capabilities

## Isolation

* Each program runs in its own sandbox
* No shared writable memory by default

## Security Guarantees

* Prevents lateral movement between applications
* Limits damage from compromised programs

## Execution Model

Programs execute under:

* Strict sandboxing
* Optional IFC enforcement
* Auditing (policy-driven)

## Use Cases

* User applications
* Plugins
* Untrusted code execution
