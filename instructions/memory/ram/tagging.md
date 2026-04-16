# Memory Tagging

## Overview

Each memory unit includes metadata for security enforcement.

## Tag Components

* Capability tag (valid/invalid)
* Security label (optional IFC)
* Domain ID

## Storage

* Stored alongside memory (shadow memory or inline)

## Usage

* Validates capability loads/stores
* Enforces IFC propagation

## Rules

* Tags cannot be modified directly by user code
* Tag updates occur via controlled instructions

## Security

* Prevents pointer forgery
* Enables fine-grained tracking

## Notes

Tagging is mandatory for capability-aware execution.
