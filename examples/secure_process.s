# Secure Process with Capability Isolation

.section .text
global _start

_start:

```
# Create sandboxed execution environment
sandbox.create r1, sandbox_cap=cap.new(PROCESS_ISOLATED)

# Enter sandbox
sandbox.enter

# Load restricted capability memory region
cap.load_region r2, region="secure_buffer"

# Write data safely inside sandbox
store r2, value=42

# Audit log entry
audit.log EVENT_PROCESS_START, data_cap=r2

# Exit sandbox
sandbox.exit

# Halt process
halt
```
