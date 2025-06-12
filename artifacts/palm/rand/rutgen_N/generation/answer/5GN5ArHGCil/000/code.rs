// Answer 0

#[derive(Debug)]
struct Rng;

impl Rng {
    fn seed_from_u64(state: u64) -> Self {
        Rng
    }
}

struct SmallRng(Rng);

impl SmallRng {
    fn seed_from_u64(state: u64) -> Self {
        SmallRng(Rng::seed_from_u64(state))
    }
}

#[test]
fn test_seed_from_u64() {
    let state: u64 = 42;
    let rng = SmallRng::seed_from_u64(state);
    assert!(std::mem::size_of_val(&rng) > 0);
}

#[test]
fn test_seed_boundary_case() {
    let rng_min = SmallRng::seed_from_u64(u64::MIN);
    assert!(std::mem::size_of_val(&rng_min) > 0);
    
    let rng_max = SmallRng::seed_from_u64(u64::MAX);
    assert!(std::mem::size_of_val(&rng_max) > 0);
}

