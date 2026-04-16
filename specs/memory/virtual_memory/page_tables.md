# Page Tables

## Overview

Maps virtual addresses to physical memory.

## Structure

* Multi-level page tables (recommended)
* Indexed by virtual address bits

## Entry Fields

* Physical address
* Permissions (R/W/X)
* Valid bit
* Domain ID
* Capability reference (optional)

## Control

* Managed by OS or hypervisor
* Protected from user modification

## Security

* No writable page tables in user space
* Updates require privileged instructions

## Notes

Page tables integrate with capability enforcement.
