// Answer 0

#[test]
fn test_reseed_success() {
    struct MockCore {
        reseed_called: bool,
    }

    impl MockCore {
        fn new() -> Self {
            MockCore { reseed_called: false }
        }

        fn reseed(&mut self) -> Result<(), ()> {
            self.reseed_called = true;
            Ok(())
        }
    }

    struct MockRng {
        core: MockCore,
    }

    impl MockRng {
        fn new() -> Self {
            MockRng {
                core: MockCore::new(),
            }
        }

        fn reset(&mut self) {
            // Mock reset behavior
        }
    }

    struct Rsdr(MockRng);

    impl Rsdr {
        fn reset(&mut self) {
            self.0.reset();
        }

        fn core(&mut self) -> &mut MockCore {
            &mut self.0.core
        }

        pub fn reseed(&mut self) -> Result<(), ()> {
            self.reset();
            self.core().reseed()
        }
    }

    let mut rng = Rsdr(MockRng::new());
    let result = rng.reseed();
    assert!(result.is_ok());
    assert!(rng.0.core.reseed_called);
}

#[test]
#[should_panic]
fn test_reseed_panics_when_failed() {
    struct FaultyCore;

    impl FaultyCore {
        fn reseed(&mut self) -> Result<(), ()> {
            Err(())
        }
    }

    struct FaultyRng {
        core: FaultyCore,
    }

    impl FaultyRng {
        fn new() -> Self {
            FaultyRng { core: FaultyCore }
        }

        fn reset(&mut self) {
            // Mock reset behavior
        }
    }

    struct Rsdr(FaultyRng);

    impl Rsdr {
        fn reset(&mut self) {
            self.0.reset();
        }

        fn core(&mut self) -> &mut FaultyCore {
            &mut self.0.core
        }

        pub fn reseed(&mut self) -> Result<(), ()> {
            self.reset();
            self.core().reseed()?;
            Ok(())
        }
    }

    let mut rng = Rsdr(FaultyRng::new());
    rng.reseed().expect("This should panic due to reseed failure");
}

