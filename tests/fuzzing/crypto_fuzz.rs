use rand::{Rng, thread_rng};

fn encrypt(key: u8, data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

#[test]
fn fuzz_crypto_roundtrip() {
    let mut rng = thread_rng();

    for _ in 0..10000 {
        let key = rng.gen::<u8>();
        let len = rng.gen_range(0..256);

        let data: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

        let enc = encrypt(key, &data);
        let dec = encrypt(key, &enc);

        assert_eq!(dec, data);
    }
}
