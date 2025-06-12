// Answer 0

#[test]
fn test_from_seed() {
    let seed: [u8; 16] = [
        1, 0, 0, 0, 0, 0, 0, 0, // first u64 (state)
        0, 0, 0, 0, 0, 0, 0, 0  // second u64 (stream)
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 1); // expected state
    assert_eq!(rng.increment, 1); // expected increment, stream is 0, so increment is (0 << 1) | 1

    let seed: [u8; 16] = [
        2, 0, 0, 0, 0, 0, 0, 0, // new state
        3, 0, 0, 0, 0, 0, 0, 0  // new stream
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 2); // expected state
    assert_eq!(rng.increment, 7); // expected increment, stream is 3, so increment is (3 << 1) | 1
}

#[test]
fn test_from_seed_odd_increment() {
    let seed: [u8; 16] = [
        0, 0, 0, 0, 0, 0, 0, 0, // first u64 (state)
        5, 0, 0, 0, 0, 0, 0, 0  // second u64 (stream; odd)
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 0); // expected state
    assert_eq!(rng.increment, 11); // expected increment, stream is 5, so increment is (5 << 1) | 1
}

