// Answer 0

#[test]
fn test_try_with_rng_success() {
    struct Rng(u32);
    
    struct RestoreOnDrop<'a> {
        rng: &'a std::thread::LocalKey<Rng>,
        current: Rng,
    }

    let result = try_with_rng(|rng: &mut Rng| {
        rng.0 += 1; // Simulate operation by incrementing the state
        rng.0
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1); // Since the initial value is 0, it should return 1
}

#[test]
#[should_panic]
fn test_try_with_rng_access_error() {
    struct Rng(u32);
    
    struct RestoreOnDrop<'a> {
        rng: &'a std::thread::LocalKey<Rng>,
        current: Rng,
    }

    // Simulate a panic by trying to access RNG from a non-local thread
    let result = std::thread::spawn(|| {
        try_with_rng(|rng: &mut Rng| {
            rng.0 += 1;
            rng.0
        })
    }).join();

    assert!(result.is_err());
}

#[test]
fn test_try_with_rng_boundary_condition() {
    struct Rng(u32);
    
    struct RestoreOnDrop<'a> {
        rng: &'a std::thread::LocalKey<Rng>,
        current: Rng,
    }

    // Set an initial value at the boundary (e.g., max value)
    RNG.try_with(|rng| {
        let mut restore = RestoreOnDrop { rng, current: Rng(u32::MAX) };

        let result = try_with_rng(|rng: &mut Rng| {
            rng.0 = 0; // Resetting to test boundary condition
            rng.0
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // Expect to reset to 0
    }).unwrap();
}

#[test]
fn test_try_with_rng_multiple_calls() {
    struct Rng(u32);
    
    struct RestoreOnDrop<'a> {
        rng: &'a std::thread::LocalKey<Rng>,
        current: Rng,
    }

    for _ in 0..10 {
        let result = try_with_rng(|rng: &mut Rng| {
            rng.0 += 1; // Increment state for each call
            rng.0
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // Each call should return 1 since we initialize Rng(0)
    }
}

