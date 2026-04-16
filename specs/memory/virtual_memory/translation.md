# Address Translation

## Overview

Translates virtual addresses into physical addresses.

## Process

1. Input virtual address
2. Lookup page table
3. Validate permissions
4. Produce physical address

## Integration with Capabilities

* Capability bounds checked BEFORE translation
* Translation must not bypass capability limits

## Failure Cases

* Page not present
* Permission violation
* Invalid mapping

## Security

* Translation cannot elevate privilege
* All results must respect domain constraints
