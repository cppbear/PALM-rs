// Answer 0

#[test]
fn test_get_seed() {
    let seed = get_seed();
    assert!(seed > 0, "Seed should be a positive number");
}

#[test]
fn test_get_seed_is_thread_local() {
    let seed1 = get_seed();
    std::thread::spawn(|| {
        let seed2 = get_seed();
        assert_ne!(seed1, seed2, "Seeds should differ across threads");
    }).join().unwrap();
}

