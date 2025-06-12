// Answer 0

#[test]
fn test_fork_unique_sequence() {
    let mut base1 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    let mut base2 = fastrand::Rng::with_seed(0x4d595df4d0f33173);

    let mut rng1 = base1.fork();
    let mut rng2 = base2.fork();

    let value1 = rng1.u32(..);
    let value2 = rng2.u32(..);

    assert_ne!(value1, value2, "Forked generators should produce different sequences");
}

#[test]
fn test_fork_state_independence() {
    let mut base = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    let mut rng1 = base.fork();
    let mut rng2 = base.fork();

    let value1_initial = rng1.u32(..);
    let value2_initial = rng2.u32(..);
    
    let value1_after = rng1.u32(..);
    let value2_after = rng2.u32(..);

    assert_ne!(value1_initial, value1_after, "Rng1 should produce different values after usage");
    assert_ne!(value2_initial, value2_after, "Rng2 should produce different values after usage");

    assert_ne!(value1_initial, value2_initial, "Initial values from different generators should differ");
    assert_ne!(value1_after, value2_after, "Final values from different generators should differ");
}

#[test]
fn test_fork_clone_greater_than_one() {
    let mut base = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    let mut rng1 = base.fork();
    let mut rng2 = rng1.fork();
    let value1 = rng1.u32(..);
    let value2 = rng2.u32(..);

    assert_ne!(value1, value2, "Forked generators should produce different values");
}

#[should_panic]
fn test_fork_with_large_seed() {
    let mut base = fastrand::Rng::with_seed(u64::MAX);
    let _ = base.fork();
} 

#[test]
fn test_fork_reproducibility() {
    let seed = 0x4d595df4d0f33173;
    let mut base1 = fastrand::Rng::with_seed(seed);
    let mut base2 = fastrand::Rng::with_seed(seed);
    
    let mut rng1 = base1.fork();
    let mut rng2 = base2.fork();

    let first_value_rng1 = rng1.u32(..);
    let first_value_rng2 = rng2.u32(..);

    assert_ne!(first_value_rng1, first_value_rng2, "Forked generators should be different even with the same seed");
}

