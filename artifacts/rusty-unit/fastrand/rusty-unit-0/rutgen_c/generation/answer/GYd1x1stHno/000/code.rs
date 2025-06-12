// Answer 0

#[test]
fn test_fork_generator() {
    let seed = 0x4d595df4d0f33173;
    let mut base1 = Rng::with_seed(seed);
    let mut base2 = Rng::with_seed(seed);

    base1.bool(); // Use the generator once.
    base2.bool(); // Use the generator once.

    let rng1 = base1.fork();
    let rng2 = base2.fork();

    assert_ne!(rng1.get_seed(), rng2.get_seed());
}

#[test]
fn test_fork_determinism() {
    let seed = 0x4d595df4d0f33173;
    let mut base = Rng::with_seed(seed);
    
    base.bool(); // Use the generator once.
    let rng1 = base.fork();
    let rng2 = base.fork();

    // Check that the two forked generators produce the same value
    let value1 = rng1.u32(..);
    let value2 = rng2.u32(..);
    
    assert_eq!(value1, value2);
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn test_fork_empty_range() {
    let seed = 0u64;
    let mut base = Rng::with_seed(seed);
    
    base.bool(); // Use the generator once.
    let mut rng1 = base.fork();
    let mut rng2 = base.fork();

    // Force a condition to create an empty range situation
    let _ = rng1.u32(1..=0);  
    let _ = rng2.u32(1..=0);
}

