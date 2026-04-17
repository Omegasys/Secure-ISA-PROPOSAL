fn encrypt(key: u8, data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

#[test]
fn test_encrypted_vm_to_vm_communication() {
    let shared_key = 0x42;

    let vm1_msg = b"hello vm2";

    // VM1 encrypts
    let ciphertext = encrypt(shared_key, vm1_msg);

    // VM2 decrypts
    let received = encrypt(shared_key, &ciphertext);

    assert_eq!(received, vm1_msg);
}
