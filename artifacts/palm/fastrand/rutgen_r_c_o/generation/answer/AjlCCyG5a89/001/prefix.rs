// Answer 0

#[test]
fn test_gen_u32_min() {
    let mut rng = Rng(0);
    let result = rng.gen_u32();
}

#[test]
fn test_gen_u32_mid_range() {
    let mut rng = Rng(1);
    let result = rng.gen_u32();
}

#[test]
fn test_gen_u32_max() {
    let mut rng = Rng(u64::MAX);
    let result = rng.gen_u32();
}

#[test]
fn test_gen_u32_large_value() {
    let mut rng = Rng(5_000_000_000);
    let result = rng.gen_u32();
}

#[test]
fn test_gen_u32_random_value() {
    let mut rng = Rng(123456789);
    let result = rng.gen_u32();
}

