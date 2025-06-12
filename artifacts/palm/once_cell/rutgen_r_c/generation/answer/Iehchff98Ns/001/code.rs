// Answer 0

#[test]
fn test_get_or_init_empty() {
    use core::num::NonZeroUsize;

    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }

        fn get(&self) -> Option<NonZeroUsize> {
            self.once.get()
        }

        fn initialize(&self, value: NonZeroUsize) {
            let _ = self.once.set(value).unwrap();
        }
    }

    // Test with a case that calls the function and initializes with a valid NonZeroUsize
    let test_instance = TestOnceNonZeroUsize::new();
    let init_value = NonZeroUsize::new(1).unwrap();

    let value = test_instance.once.get_or_init(|| init_value);
    assert_eq!(value.get(), init_value.get());
}

#[test]
#[should_panic]
fn test_get_or_init_multiple_initializations() {
    use core::num::NonZeroUsize;
    use std::sync::{Arc, Mutex};
    use std::thread;

    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let test_instance = Arc::new(TestOnceNonZeroUsize::new());

    let handles: Vec<_> = (0..10).map(|_| {
        let instance_clone = Arc::clone(&test_instance);
        thread::spawn(move || {
            instance_clone.once.get_or_init(|| {
                // Simulate an initialization function that may cause a panic.
                NonZeroUsize::new(1).unwrap()
            });
        })
    }).collect();

    // Join all threads and trigger multiple calls to get_or_init
    for handle in handles {
        handle.join().unwrap();
    }
}

