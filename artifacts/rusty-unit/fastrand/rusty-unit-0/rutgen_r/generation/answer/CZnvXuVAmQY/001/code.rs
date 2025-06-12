// Answer 0

#[test]
fn test_with_rng_valid() {
    struct Rng(i32);
    struct RestoreOnDrop<'a> {
        rng: &'a std::cell::RefCell<Rng>,
        current: Rng,
    }
    
    fn mock_with<F>(f: F) -> Rng 
    where
        F: FnOnce(&mut Rng) -> Rng,
    {
        let rng = std::cell::RefCell::new(Rng(42));
        let current = rng.replace(Rng(0));
        let mut restore = RestoreOnDrop { rng: &rng, current };
        f(&mut restore.current)
    }

    let result = mock_with(|rng| {
        rng.0 += 10;
        Rng(rng.0)
    });

    assert_eq!(result.0, 52);
}

#[test]
#[should_panic]
fn test_with_rng_panic() {
    struct Rng(i32);
    struct RestoreOnDrop<'a> {
        rng: &'a std::cell::RefCell<Rng>,
        current: Rng,
    }
    
    fn mock_with<F>(f: F)
    where
        F: FnOnce(&mut Rng) -> Rng,
    {
        let rng = std::cell::RefCell::new(Rng(42));
        let current = rng.replace(Rng(0));
        let _restore = RestoreOnDrop { rng: &rng, current };
        f(&mut Rng(-1)) // This should trigger a panic if the function expects a valid range
    }

    mock_with(|rng| {
        // Triggering panic condition by using a negative value
        assert!(rng.0 >= 0);
        Rng(rng.0)
    });
}

