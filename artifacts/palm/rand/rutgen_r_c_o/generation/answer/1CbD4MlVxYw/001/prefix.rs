// Answer 0

#[test]
fn test_from_seed_all_zero() {
    let seed: [u8; 32] = [0; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_non_zero() {
    let seed: [u8; 32] = [1; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_alternating() {
    let seed: [u8; 32] = [0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_large_value() {
    let seed: [u8; 32] = [255; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_edge_case() {
    let seed: [u8; 32] = [0; 31].iter().cycle().take(32).cloned().collect::<Vec<u8>>().try_into().unwrap();
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

