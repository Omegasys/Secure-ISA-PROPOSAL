Instruction Encoding
Base Format (32-bit fixed width)
Field	Bits	Description
Opcode	7	Instruction identifier
Format	3	Encoding format type
rd	5	Destination register
rs1	5	Source register 1
rs2/imm	12	Source register 2 or immediate
Extended Format (64-bit)

Used for:

Capability manipulation
Cryptographic operations
System instructions
Format Types
R: Register
I: Immediate
M: Memory
C: Capability
S: System
Security Hooks

Each instruction implicitly triggers:

Capability validation
Domain isolation check
Optional IFC label propagation
