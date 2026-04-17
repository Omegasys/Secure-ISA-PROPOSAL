#[derive(Clone)]
struct Ratchet {
    state: u8,
}

fn advance(r: &mut Ratchet) {
    r.state = r.state.wrapping_mul(3) ^ 0xAA;
}

#[test]
fn test_state_progression() {
    let mut r = Ratchet { state: 1 };

    let initial = r.state;
    advance(&mut r);

    assert_ne!(r.state, initial);
}

#[test]
fn test_multiple_advances_unique() {
    let mut r = Ratchet { state: 1 };

    let mut states = vec![];

    for _ in 0..5 {
        advance(&mut r);
        states.push(r.state);
    }

    for i in 0..states.len() {
        for j in i+1..states.len() {
            assert_ne!(states[i], states[j]);
        }
    }
}
