# Encrypted Communication Example (Hybrid Crypto + Ratchet)

.section .data
pubkey_cap: cap.load_public_key "peer_public"
privkey_cap: cap.load_private_key "local_private"
msg_cap:     cap.load_string "Secret Message"
out_cap:     cap.alloc 256

.section .text
global _start

_start:

```
# Initialize hybrid encrypted channel
crypto.hybrid pubkey_cap, symkey_cap=r1, data_cap=msg_cap, out_cap=out_cap

# Initialize ratchet session for forward secrecy
crypto.ratchet.init privkey_cap, session_cap=r2

# Encrypt message using ratchet
crypto.ratchet.message r2, msg_cap, out_cap

# Send encrypted output over network device
io.output out_cap, device_id=NET_DEVICE, size=256

# Rotate ratchet state for next message
crypto.ratchet.advance r2

# Audit encryption event
audit.log EVENT_CRYPTO_TX, data_cap=out_cap

halt
```
