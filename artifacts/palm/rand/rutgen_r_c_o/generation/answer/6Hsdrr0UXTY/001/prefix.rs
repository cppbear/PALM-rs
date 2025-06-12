// Answer 0

#[test]
fn test_from_seed_min_value() {
    let seed: [u8; 16] = [0; 16];
    let rng = Mcg128Xsl64::from_seed(seed);
}

#[test]
fn test_from_seed_max_value() {
    let seed: [u8; 16] = [255; 16];
    let rng = Mcg128Xsl64::from_seed(seed);
}

#[test]
fn test_from_seed_mid_value() {
    let seed: [u8; 16] = [128; 16];
    let rng = Mcg128Xsl64::from_seed(seed);
}

#[test]
fn test_from_seed_random_value_1() {
    let seed: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let rng = Mcg128Xsl64::from_seed(seed);
}

#[test]
fn test_from_seed_random_value_2() {
    let seed: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let rng = Mcg128Xsl64::from_seed(seed);
}

