# Virtual Address Space

## Overview

Defines per-process virtual memory layout.

## Structure

* Each process has its own address space
* Isolated by default

## Regions

* Code
* Heap
* Stack
* Shared memory (optional)

## Address Size

* Depends on ISA bit-width

## Security

* No direct mapping to physical memory
* All mappings validated by kernel/hypervisor

## Notes

Virtual addresses are meaningless without translation.
