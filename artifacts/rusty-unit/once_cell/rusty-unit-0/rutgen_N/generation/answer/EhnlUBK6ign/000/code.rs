// Answer 0

#[test]
fn test_force_initialization() {
    use once_cell::unsync::Lazy;

    let lazy = Lazy::new(|| 42);
    
    assert_eq!(force(&lazy), &42);
    assert_eq!(&*lazy, &42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_poisoned() {
    use once_cell::unsync::Lazy;

    struct Poisoned {
        initialized: bool,
    }

    impl Poisoned {
        fn new() -> Self {
            Poisoned { initialized: false }
        }

        fn reset(&mut self) {
            self.initialized = false;
        }
    }

    let lazy = Lazy::new(|| {
        if !initialized {
            panic!("poisoned");
        }
        99
    });

    // Force initialization to poison the lazy value.
    lazy.force();

    // Now attempt to force it again which should panic.
    force(&lazy);
}

