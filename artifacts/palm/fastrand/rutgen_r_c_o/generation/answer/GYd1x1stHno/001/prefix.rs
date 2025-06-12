// Answer 0

#[test]
fn test_fork_with_min_seed() {
    let mut rng = fastrand::Rng::with_seed(0);
    rng.fork();
}

#[test]
fn test_fork_with_max_seed() {
    let mut rng = fastrand::Rng::with_seed(u64::MAX);
    rng.fork();
}

#[test]
fn test_fork_with_random_seed() {
    let mut rng = fastrand::Rng::with_seed(123456789);
    rng.fork();
}

#[test]
fn test_fork_after_one_use() {
    let mut rng = fastrand::Rng::with_seed(42);
    let _ = rng.bool(); // Use the generator
    let _ = rng.fork();
}

#[test]
fn test_fork_multiple_times() {
    let mut rng = fastrand::Rng::with_seed(987654321);
    for _ in 0..10 {
        let _ = rng.fork();
    }
}

#[test]
fn test_fork_with_identical_seeds() {
    let mut base_rng = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    let mut fork1 = base_rng.fork();
    let mut fork2 = base_rng.fork();

    let _ = fork1.bool();
    let _ = fork2.bool();
}

