// Answer 0

#[test]
fn test_fork_creates_different_generators() {
    let mut base1 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    let mut base2 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    
    // Use the generator to advance state
    base1.bool();
    base2.bool();
    
    // Create forks from both base generators
    let mut rng1 = base1.fork();
    let mut rng2 = base2.fork();
    
    // Ensure the two forked generators produce different values
    let value1 = rng1.u32(..);
    let value2 = rng2.u32(..);
    
    assert_ne!(value1, value2);
}

#[test]
fn test_fork_generates_deterministic_sequence() {
    let mut base = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    
    // Create a fork
    let mut rng1 = base.fork();
    
    // Get a sequence of values from the fork
    let seq: Vec<u32> = (0..10).map(|_| rng1.u32(..)).collect();
    
    // Create another fork from the same base generator
    let mut rng2 = base.fork();
    
    // Get the same sequence of values from the second fork
    let seq_clone: Vec<u32> = (0..10).map(|_| rng2.u32(..)).collect();
    
    assert_eq!(seq, seq_clone);
}

