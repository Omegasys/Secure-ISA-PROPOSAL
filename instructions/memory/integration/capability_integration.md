# Capability Integration with Memory

## Overview

Defines how capabilities interact with memory system.

## Rules

* All memory access requires capability
* Capabilities define:

  * Base
  * Bounds
  * Permissions

## Enforcement Order

1. Capability check
2. Address translation
3. Physical access

## Restrictions

* No raw pointer dereferencing
* No bypass of capability checks

## Security

* Prevents buffer overflow
* Prevents arbitrary memory access

## Notes

Capabilities are the primary memory protection mechanism.
