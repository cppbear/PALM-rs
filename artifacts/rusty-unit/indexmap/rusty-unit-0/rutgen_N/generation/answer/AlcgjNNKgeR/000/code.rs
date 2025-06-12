// Answer 0

#[test]
fn test_get() {
    struct Wrapper(u32);

    impl Wrapper {
        fn get(self) -> u64 {
            self.0 as u64
        }
    }

    let wrapper = Wrapper(42);
    assert_eq!(wrapper.get(), 42);
}

#[test]
fn test_get_zero() {
    struct Wrapper(u32);

    impl Wrapper {
        fn get(self) -> u64 {
            self.0 as u64
        }
    }

    let wrapper = Wrapper(0);
    assert_eq!(wrapper.get(), 0);
}

#[test]
fn test_get_large_value() {
    struct Wrapper(u32);

    impl Wrapper {
        fn get(self) -> u64 {
            self.0 as u64
        }
    }

    let wrapper = Wrapper(u32::MAX);
    assert_eq!(wrapper.get(), u32::MAX as u64);
}

