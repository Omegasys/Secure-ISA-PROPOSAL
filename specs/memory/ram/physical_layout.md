# Physical Memory Layout

## Overview

Defines how physical RAM is structured and accessed by the system.

## Addressing

* Physical addresses are linear
* Size depends on bit-width configuration

## Regions

* Reserved (firmware / ROM)
* Kernel memory
* User memory
* Device-mapped memory

## Alignment

* Word-aligned access required
* Cache-line alignment recommended

## Security

* No direct access without capability
* Regions may be locked (immutable)

## Notes

Physical memory is NEVER accessed directly by user programs.
All access must go through capability-checked pathways.
