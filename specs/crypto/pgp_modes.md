# PGP-Style Modes

## Overview

The ISA supports primitives required to implement PGP-style encryption workflows.

## Design Principle

Provide building blocks for:

* Encryption
* Signing
* Key wrapping

NOT full PGP protocol implementation.

---

## Core Operations

### 1. Encrypt + Sign

#### Flow

1. Encrypt data with symmetric key
2. Encrypt symmetric key with recipient public key
3. Sign message with sender private key

#### Hardware Support

* Symmetric encryption
* Asymmetric encryption
* Digital signature instructions

---

### 2. Sign Only

#### Flow

1. Hash message
2. Sign hash

#### Hardware Support

* Secure hash
* Signature generation

---

### 3. Verify

#### Flow

1. Hash message
2. Verify signature

---

## Key Wrapping

### Purpose

* Protect symmetric keys during transport

### Support

* Dedicated key wrapping instructions
* Secure key unwrapping

---

## Metadata Handling

Handled in software:

* Message headers
* Key IDs
* Compression

---

## Security Considerations

* All cryptographic operations MUST be constant-time
* Keys MUST remain in protected storage
* No exposure through general-purpose registers

---

## Non-Goals

* Email integration
* Trust models (web-of-trust)
* Certificate management
