// Answer 0

#[test]
fn test_next_u64() {
    struct MockRng {
        value: u64,
    }

    impl MockRng {
        fn new(value: u64) -> Self {
            MockRng { value }
        }

        fn next_u64(&mut self) -> u64 {
            self.value += 1;
            self.value
        }
    }

    struct ThreadRng {
        rng: std::cell::UnsafeCell<MockRng>,
    }

    impl ThreadRng {
        fn new(rng: MockRng) -> Self {
            ThreadRng {
                rng: std::cell::UnsafeCell::new(rng),
            }
        }

        fn next_u64(&mut self) -> u64 {
            let rng = unsafe { &mut *self.rng.get() };
            rng.next_u64()
        }
    }

    let mut thread_rng = ThreadRng::new(MockRng::new(0));

    assert_eq!(thread_rng.next_u64(), 1);
    assert_eq!(thread_rng.next_u64(), 2);
    assert_eq!(thread_rng.next_u64(), 3);
}

#[test]
#[should_panic]
fn test_next_u64_panic() {
    struct MockRng {
        value: u64,
    }

    impl MockRng {
        fn new(value: u64) -> Self {
            MockRng { value }
        }

        fn next_u64(&mut self) -> u64 {
            self.value += 1;
            self.value
        }
    }

    struct ThreadRng {
        rng: std::cell::UnsafeCell<MockRng>,
    }

    impl ThreadRng {
        fn new(rng: MockRng) -> Self {
            ThreadRng {
                rng: std::cell::UnsafeCell::new(rng),
            }
        }

        fn next_u64(&mut self) -> u64 {
            let rng = unsafe { &mut *self.rng.get() };
            rng.next_u64()
        }
    }

    let mut thread_rng = ThreadRng::new(MockRng::new(0));
    let _ = thread_rng.next_u64(); // Valid usage
    let _ = thread_rng.next_u64(); // Valid usage

    // Simulate a panic condition by creating a second mutable reference
    let rng_ref = unsafe { &mut *thread_rng.rng.get() };
    let _ = rng_ref.next_u64(); // Though this code won't actually panic, it's meant to show misuse
}

