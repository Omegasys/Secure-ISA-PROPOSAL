Threat Model
Assumptions

The system assumes adversaries may have:

User-level access
Kernel-level access (in compromised scenarios)
Physical access to hardware
Ability to observe timing and side channels
Threat Categories
Memory Safety Attacks
Buffer overflows
Use-after-free
Arbitrary memory access

Mitigation:

Capability-based memory
Tagged memory
Bounds enforcement
Privilege Escalation
Exploiting bugs to gain higher privileges

Mitigation:

Multi-layer privilege hierarchy
Hardware-enforced transitions
Capability validation
Side-Channel Attacks
Timing attacks
Cache attacks
Speculative execution leaks

Mitigation:

Constant-time execution modes
Partitioned microarchitecture
Optional deterministic execution
Code Injection
Executing unauthorized code

Mitigation:

Signed code enforcement
Immutable execution regions
Secure boot chain
Physical Attacks
Memory probing
Replay attacks

Mitigation:

Memory encryption
Integrity verification
Anti-replay mechanisms
