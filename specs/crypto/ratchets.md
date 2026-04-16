# Ratchet Mechanisms

## Overview

The ISA provides **primitive support** for implementing ratcheting key systems (e.g., forward secrecy protocols), without embedding full protocol logic.

## Design Principle

Ratchets are implemented in software, with hardware support for:

* Secure key evolution
* Atomic state updates
* Protected key storage

## Supported Ratchet Types

### 1. Symmetric-Key Ratchet

#### Operation

* Key evolves via one-way function:
  K(n+1) = Hash(K(n))

#### Hardware Support

* Fast secure hash operations
* Key overwrite guarantees
* Atomic update instruction

---

### 2. Asymmetric Ratchet

#### Operation

* Periodic key pair rotation
* Secure key exchange per step

#### Hardware Support

* Fast key generation
* Secure key destruction
* Isolated key slots

---

## Double Ratchet Support

The ISA enables:

* Combined symmetric + asymmetric ratchets
* Independent state tracking

### Required Features

* Multiple key slots
* Secure state transitions
* Isolation between sessions

---

## Security Properties

* Forward secrecy (past keys unrecoverable)
* Post-compromise security (future keys recoverable after reset)
* Key erasure guarantees

---

## Implementation Notes

* Ratchet logic resides in software
* ISA ensures:

  * Secure primitives
  * Safe key lifecycle
  * No key reuse leakage

## Non-Goals

* Message protocol handling
* Network synchronization
