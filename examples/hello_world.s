# Hello World Example for Secure ISA

.section .data
msg_cap:    cap.load_string "Hello, Secure ISA\n"

.section .text
global _start

_start:
# Prepare output buffer capability
cap.load r1, msg_cap

```
# Write to output device (stdout = device 1)
output r1, device_id=1, size=18

# Exit via syscall
syscall SYS_EXIT, arg_cap=null
```
