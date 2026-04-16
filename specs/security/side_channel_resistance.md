# Side-Channel Resistance

## Overview

The ISA includes mechanisms to mitigate timing, power, and microarchitectural side-channel attacks.

## Threat Model

* Timing attacks
* Cache-based attacks
* Speculative execution attacks
* Power analysis

## Mitigations

### Constant-Time Execution

* Fixed latency instructions
* No data-dependent branching (optional mode)

### Memory Access

* Uniform memory access timing
* Cache partitioning or disabling

### Speculation Control

* Speculation barriers
* Optional full speculation disable

### Noise Injection (Optional)

* Timing jitter
* Power masking (hardware-dependent)

## ISA Support

* Instructions to enable/disable protections
* Security mode flags

## Tradeoffs

* Reduced performance
* Increased power usage

## Use Cases

* Cryptographic operations
* Secure enclaves
* Sensitive data processing
