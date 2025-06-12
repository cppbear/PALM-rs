// Answer 0

#[test]
fn test_from_seed_all_zero() {
    let seed: [u8; 32] = [0; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_eq!(rng.s, [0; 4]); // This should be the state when seed was all zero
}

#[test]
fn test_from_seed_non_zero() {
    let seed: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng.s, [0; 4]); // This should not be all zeros
}

#[test]
fn test_from_seed_edge_case_single_non_zero() {
    let seed: [u8; 32] = [0; 31][..].iter().chain(&[1]).cloned().collect::<Vec<u8>>().try_into().unwrap();
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng.s, [0; 4]); // Ensures state is not all zeros due to one non-zero byte
}

#[should_panic]
fn test_from_seed_panic_on_all_zero() {
    let seed: [u8; 32] = [0; 32];
    let _ = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_boundary_case() {
    let seed: [u8; 32] = [u8::MAX; 32]; // All bytes set to maximum value
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng.s, [0; 4]); // This should not be all zeros
}

