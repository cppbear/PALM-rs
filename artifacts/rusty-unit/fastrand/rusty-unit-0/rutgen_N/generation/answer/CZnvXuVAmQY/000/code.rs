// Answer 0

#[test]
fn test_with_rng() {
    struct Rng(u32);
    struct RestoreOnDrop<'a> {
        rng: &'a std::cell::RefCell<Rng>,
        current: Rng,
    }

    impl Drop for RestoreOnDrop<'_> {
        fn drop(&mut self) {
            self.rng.replace(self.current);
        }
    }

    let rng = std::cell::RefCell::new(Rng(1));
    std::thread::local::LocalKey::new(|| rng);

    let result = with_rng(|current_rng| {
        assert_eq!(current_rng.0, 0); // Check that the initial value is overridden to 0
        current_rng.0 + 1 // Return a value to test the overall functionality
    });

    assert_eq!(result, 1); // Verify the result after calling with_rng
}

#[test]
fn test_with_rng_restore() {
    struct Rng(u32);
    struct RestoreOnDrop<'a> {
        rng: &'a std::cell::RefCell<Rng>,
        current: Rng,
    }

    impl Drop for RestoreOnDrop<'_> {
        fn drop(&mut self) {
            self.rng.replace(self.current);
        }
    }

    let rng = std::cell::RefCell::new(Rng(2));
    std::thread::local::LocalKey::new(|| rng);

    with_rng(|current_rng| {
        current_rng.0 = 5; // Change the value within the thread-local generator.
    });

    // Ensure that the original value is restored.
    assert_eq!(rng.borrow().0, 2);
}

#[test]
fn test_with_rng_multiple_calls() {
    struct Rng(u32);
    struct RestoreOnDrop<'a> {
        rng: &'a std::cell::RefCell<Rng>,
        current: Rng,
    }

    impl Drop for RestoreOnDrop<'_> {
        fn drop(&mut self) {
            self.rng.replace(self.current);
        }
    }

    let rng = std::cell::RefCell::new(Rng(3));
    std::thread::local::LocalKey::new(|| rng);

    for i in 0..3 {
        let result = with_rng(|current_rng| {
            current_rng.0 = i; // Update the rng value
            current_rng.0 * 2 // Return a calculated result
        });
        assert_eq!(result, i * 2); // Test that the returned result is as expected
    }

    // Verify that original RNG value is restored
    assert_eq!(rng.borrow().0, 3);
}

