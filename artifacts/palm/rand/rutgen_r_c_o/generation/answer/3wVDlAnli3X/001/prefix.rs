// Answer 0

#[test]
fn test_from_seed_zero_state() {
    let seed: [u8; 16] = [0; 16];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_max_state() {
    let seed: [u8; 16] = [
        255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255,
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_odd_increment() {
    let seed: [u8; 16] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_large_odd_increment() {
    let seed: [u8; 16] = [
        0, 0, 0, 0, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 1,
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_varying_increment() {
    let seed: [u8; 16] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 3,
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_all_even() {
    let seed: [u8; 16] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 2,
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_randomized() {
    let seed: [u8; 16] = [
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 15,
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

