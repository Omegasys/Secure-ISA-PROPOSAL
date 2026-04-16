# Memory Context Switching

## Overview

Defines how memory state changes during context switches.

## Components Switched

* Page tables
* TLB entries (flush or partition)
* Capability registers

## Steps

1. Save current context
2. Load new context
3. Update memory mappings
4. Invalidate stale entries

## Security

* No leakage between contexts
* Strict domain separation

## Performance Considerations

* Minimize TLB flush cost
* Use tagged TLB if possible

## Notes

Context switching must preserve isolation guarantees.
