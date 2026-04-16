# Key Management

## Overview

The ISA provides hardware-enforced key management to ensure secure lifecycle handling of cryptographic keys.

## Design Goals

* Prevent key leakage
* Enforce key isolation
* Support secure key lifecycle

---

## Key Storage

### Secure Key Slots

* Hardware-protected
* Not directly readable by software
* Indexed access only

### Properties

* Non-exportable (by default)
* Bound to domain and privilege level
* Tagged with usage policy

---

## Key Types

* Symmetric keys
* Asymmetric private keys
* Public keys (optionally protected)
* Ephemeral session keys

---

## Key Lifecycle

### 1. Generation

* Hardware RNG-based
* Generated directly into secure slot

### 2. Usage

* Referenced by handle (not raw value)
* Used via cryptographic instructions

### 3. Rotation

* Replace or evolve key
* Atomic update support

### 4. Revocation

* Invalidate key slot
* Immediate unusability

### 5. Destruction

* Secure erase (zeroization)
* Guaranteed removal

---

## Access Control

Key usage requires:

* Valid capability
* Matching domain
* Correct privilege level

---

## Isolation

* Keys cannot be shared across domains without explicit delegation
* No raw key extraction unless explicitly permitted

---

## Audit Integration

Optional logging for:

* Key creation
* Key usage
* Key destruction

---

## Security Guarantees

* Keys never appear in general-purpose registers
* Hardware-enforced access control
* Protection against memory scraping

---

## Deterministic Mode Support

* Keys may be derived from deterministic seeds (for reproducibility)
* Must be explicitly enabled

---

## Non-Goals

* Key distribution protocols
* Certificate authority logic
* External trust management
