// Answer 0

#[test]
fn test_gen_mod_u64_case_lo_equals_n() {
    let mut rng = Rng(0);
    let n = 1;
    let result = rng.gen_mod_u64(n);
}

#[test]
fn test_gen_mod_u64_case_n_max() {
    let mut rng = Rng(0);
    let n = u64::MAX;
    let result = rng.gen_mod_u64(n);
}

#[test]
#[should_panic]
fn test_gen_mod_u64_case_n_zero() {
    let mut rng = Rng(0);
    let n = 0;
    let result = rng.gen_mod_u64(n);
}

#[test]
fn test_gen_mod_u64_case_n_large() {
    let mut rng = Rng(0);
    let n = 123456789012345678;
    let result = rng.gen_mod_u64(n);
}

#[test]
fn test_gen_mod_u64_case_n_small() {
    let mut rng = Rng(0);
    let n = 2;
    let result = rng.gen_mod_u64(n);
}

