// Answer 0

#[test]
fn test_gen_mod_u32_with_n_zero() {
    let mut rng = fastrand::Rng::new();
    let result = rng.gen_mod_u32(0);
    assert_eq!(result, 0);
}

#[test]
fn test_gen_mod_u32_with_n_one() {
    let mut rng = fastrand::Rng::new();
    let result = rng.gen_mod_u32(1);
    assert_eq!(result, 0);
}

#[test]
fn test_gen_mod_u32_with_large_n() {
    let mut rng = fastrand::Rng::new();
    let n: u32 = 1_000_000;
    let result = rng.gen_mod_u32(n);
    assert!(result < n);
}

#[test]
fn test_gen_mod_u32_partition_boundary_condition() {
    let mut rng = fastrand::Rng::new();
    let n: u32 = 100;
    let mut lo: u32;
    let mut t = n.wrapping_neg() % n;

    loop {
        lo = rng.gen_u32().wrapping_mul(n);
        if lo < t {
            break;
        }
    }

    let result = rng.gen_mod_u32(n);
    assert!(result < n);
}

#[test]
fn test_gen_mod_u32_lo_equals_t() {
    let mut rng = fastrand::Rng::new();
    let n: u32 = 100;
    let mut r = rng.gen_u32();
    let mut lo = r.wrapping_mul(n);
    let t = n.wrapping_neg() % n;

    // Adjust to ensure lo == t
    while lo != t {
        r = rng.gen_u32();
        lo = r.wrapping_mul(n);
    }

    let result = rng.gen_mod_u32(n);
    assert!(result < n);
}

