# 32-bit Variant

## Overview

The 32-bit configuration is optimized for:

* Embedded systems
* Low-power environments
* Resource-constrained secure devices

## Characteristics

* Register width: 32 bits
* Address space: 4 GB
* Capability size: compact (64–96 bits)

## Register Model

* 32 general-purpose registers (32-bit)
* 16 capability registers (compressed format)

## Memory Model

* 32-bit addressing
* Tagged memory supported (reduced metadata size)

## Performance Profile

* Low power consumption
* Reduced hardware complexity
* Faster context switching

## Security Considerations

* Smaller address space reduces entropy
* Capability compression may reduce precision
* Recommended for controlled environments

## Use Cases

* IoT devices
* Secure microcontrollers
* Embedded cryptographic modules
