// Answer 0

#[test]
fn test_gen_mod_u64_with_small_n() {
    let mut rng = Rng(12345);
    let result = rng.gen_mod_u64(10);
    assert!(result < 10);
}

#[test]
fn test_gen_mod_u64_with_large_n() {
    let mut rng = Rng(12345);
    let result = rng.gen_mod_u64(1_000_000);
    assert!(result < 1_000_000);
}

#[test]
fn test_gen_mod_u64_with_zero() {
    let mut rng = Rng(12345);
    let result = std::panic::catch_unwind(|| {
        rng.gen_mod_u64(0);
    });
    assert!(result.is_err());
}

#[test]
fn test_gen_mod_u64_with_one() {
    let mut rng = Rng(12345);
    let result = rng.gen_mod_u64(1);
    assert_eq!(result, 0);
}

#[test]
fn test_gen_mod_u64_with_boundary_value() {
    let mut rng = Rng(12345);
    let result = rng.gen_mod_u64(u64::MAX);
    assert!(result < u64::MAX);
}

