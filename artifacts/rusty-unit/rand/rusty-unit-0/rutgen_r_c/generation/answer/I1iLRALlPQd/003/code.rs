// Answer 0

#[test]
fn test_seed_from_u64_with_zero() {
    let rng = Xoshiro256PlusPlus::seed_from_u64(0);
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_seed_from_u64_with_positive_seed() {
    let rng = Xoshiro256PlusPlus::seed_from_u64(1);
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_seed_from_u64_with_large_seed() {
    let rng = Xoshiro256PlusPlus::seed_from_u64(0xffffffffffffffff);
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_seed_from_u64_with_determined_seed() {
    let rng1 = Xoshiro256PlusPlus::seed_from_u64(123);
    let rng2 = Xoshiro256PlusPlus::seed_from_u64(123);
    assert_eq!(rng1, rng2);
} 

#[test]
fn test_seed_from_u64_same_initial_state() {
    let rng1 = Xoshiro256PlusPlus::seed_from_u64(10);
    let rng2 = Xoshiro256PlusPlus::seed_from_u64(10);
    assert_eq!(rng1.s, rng2.s);
} 

#[test]
#[should_panic]
fn test_seed_from_u64_panic_on_zero_state() {
    let rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let expected = [0; 4];
    assert_eq!(rng.s, expected);
}

