# Translation Lookaside Buffer (TLB)

## Overview

Caches virtual-to-physical translations.

## Structure

* Small associative cache
* Stores recent mappings

## Fields

* Virtual address
* Physical address
* Permissions
* Domain ID

## Security

* TLB entries are domain-scoped
* No cross-domain reuse

## Invalidation

* On context switch
* On page table update

## Risks & Mitigations

* Side-channel leakage → mitigated via partitioning or flush

## Notes

TLB behavior must be deterministic in secure mode.
