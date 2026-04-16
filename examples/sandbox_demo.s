# Sandbox Isolation Demonstration

.section .text
global _start

_start:

```
# Create sandbox with restricted I/O and memory
sandbox.create r1, sandbox_cap=cap.restrict(IO=false, NET=false)

# Enter sandboxed environment
sandbox.enter

# Attempt memory allocation inside sandbox
cap.alloc r2, size=1024

# Try restricted I/O (should be blocked by policy)
# This will fail due to sandbox policy
output r2, device_id=1, size=10

# Log sandbox restriction event
audit.log EVENT_SANDBOX_VIOLATION, data_cap=r2

# Exit sandbox
sandbox.exit

halt
```
