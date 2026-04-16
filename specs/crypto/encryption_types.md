# Encryption Types

## Overview

The ISA provides hardware-accelerated cryptographic primitives rather than full protocols.

## Design Principle

The ISA exposes **secure primitives**, not complete encryption schemes.

## Supported Categories

### 1. Symmetric Encryption

#### Features

* Block cipher support (e.g., AES-class)
* Stream cipher support (optional)
* Hardware acceleration

#### Modes (primitive-level support)

* ECB (discouraged, debug only)
* CBC
* CTR
* GCM (authenticated encryption)

#### Requirements

* Constant-time execution
* Key isolation
* No key leakage through registers

---

### 2. Asymmetric Encryption

#### Features

* Public/private key operations
* Elliptic curve support (preferred)
* Modular arithmetic acceleration

#### Operations

* Encrypt / Decrypt
* Sign / Verify
* Key agreement (e.g., Diffie-Hellman class)

#### Requirements

* Side-channel resistance
* Secure key storage

---

### 3. Hybrid Encryption

#### Model

* Combine asymmetric + symmetric

#### Flow

1. Generate symmetric session key
2. Encrypt data with symmetric cipher
3. Encrypt session key with asymmetric key

#### ISA Role

* Accelerate both components
* Provide secure key handling

---

## Randomness

### Hardware RNG

* Cryptographically secure
* Seed isolation
* Deterministic override (for testing)

---

## Security Guarantees

* Constant-time primitives
* No direct key exposure
* Hardware-enforced isolation

## Non-Goals

* Full protocol implementation
* Certificate handling
* Network-level encryption
