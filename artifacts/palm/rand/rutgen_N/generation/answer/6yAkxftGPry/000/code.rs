// Answer 0

#[derive(Debug)]
struct R;

impl R {
    fn seed_from_u64(seed: u64) -> Self {
        R
    }
}

struct Seeded {
    r: R,
}

impl Seeded {
    fn new(r: R) -> Self {
        Seeded { r }
    }
    
    fn seed_from_u64(seed: u64) -> Self {
        Self::new(R::seed_from_u64(seed))
    }
}

#[test]
fn test_seed_from_u64() {
    let seed_value: u64 = 42;
    let seeded_instance = Seeded::seed_from_u64(seed_value);
    assert!(matches!(seeded_instance, Seeded { r: _ }));
}

#[test]
fn test_seed_from_u64_boundary() {
    let seed_value_min: u64 = 0;
    let seeded_instance_min = Seeded::seed_from_u64(seed_value_min);
    assert!(matches!(seeded_instance_min, Seeded { r: _ }));

    let seed_value_max: u64 = u64::MAX;
    let seeded_instance_max = Seeded::seed_from_u64(seed_value_max);
    assert!(matches!(seeded_instance_max, Seeded { r: _ }));
}

