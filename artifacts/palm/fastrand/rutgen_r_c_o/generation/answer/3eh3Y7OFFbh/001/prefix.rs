// Answer 0

#[test]
fn test_gen_u128_normal_case() {
    let mut rng = Rng(42);
    let _result = rng.gen_u128();
}

#[test]
fn test_gen_u128_another_normal_case() {
    let mut rng = Rng(12345);
    let _result = rng.gen_u128();
}

#[test]
fn test_gen_u128_with_zero_seed() {
    let mut rng = Rng(0);
    let _result = rng.gen_u128();
}

#[test]
fn test_gen_u128_large_seed() {
    let mut rng = Rng(u64::MAX);
    let _result = rng.gen_u128();
}

#[test]
fn test_gen_u128_small_seed() {
    let mut rng = Rng(1);
    let _result = rng.gen_u128();
}

