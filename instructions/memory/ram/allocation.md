# Physical Memory Allocation

## Overview

Controls allocation of physical memory pages.

## Allocation Model

* Page-based allocation
* Fixed page sizes (e.g., 4KB, 16KB, configurable)

## Allocation Sources

* Kernel allocator
* Hypervisor allocator
* Secure enclave allocator

## Constraints

* Allocation must respect domain boundaries
* Pages are zeroed before reuse

## Security

* No memory reuse without clearing
* Allocation tracked per domain

## Fragmentation Handling

* Free lists
* Buddy allocator (recommended)

## Notes

Only privileged layers can allocate raw physical memory.
