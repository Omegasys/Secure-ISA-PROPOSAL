use std::collections::HashSet;

#[derive(Default)]
struct Receiver {
    seen: HashSet<u64>,
}

fn receive(receiver: &mut Receiver, msg_id: u64) -> bool {
    if receiver.seen.contains(&msg_id) {
        return false; // replay detected
    }

    receiver.seen.insert(msg_id);
    true
}

#[test]
fn test_replay_attack_detected() {
    let mut r = Receiver::default();

    assert!(receive(&mut r, 1));
    assert!(!receive(&mut r, 1)); // replay
}
