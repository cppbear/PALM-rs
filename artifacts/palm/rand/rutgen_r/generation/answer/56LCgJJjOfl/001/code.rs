// Answer 0

#[test]
fn test_fill_bytes_with_valid_buffer() {
    struct MockRng {
        bytes: [u8; 10],
    }

    impl MockRng {
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.copy_from_slice(&self.bytes);
        }
    }

    struct RngWrapper {
        rng: std::cell::UnsafeCell<MockRng>,
    }

    impl RngWrapper {
        fn new(bytes: [u8; 10]) -> Self {
            Self {
                rng: std::cell::UnsafeCell::new(MockRng { bytes }),
            }
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let rng = unsafe { &mut *self.rng.get() };
            rng.fill_bytes(dest);
        }
    }

    let mut buffer = [0u8; 10];
    let mut rng_wrapper = RngWrapper::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    rng_wrapper.fill_bytes(&mut buffer);

    assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
#[should_panic]
fn test_fill_bytes_with_null_buffer() {
    struct MockRng;

    impl MockRng {
        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            panic!("Attempting to fill bytes on null buffer");
        }
    }

    struct RngWrapper {
        rng: std::cell::UnsafeCell<MockRng>,
    }

    impl RngWrapper {
        fn new() -> Self {
            Self {
                rng: std::cell::UnsafeCell::new(MockRng),
            }
        }

        fn fill_bytes(&mut self, dest: Option<&mut [u8]>) {
            let rng = unsafe { &mut *self.rng.get() };
            if let Some(buffer) = dest {
                rng.fill_bytes(buffer);
            } else {
                panic!("Null buffer passed to fill_bytes");
            }
        }
    }

    let mut rng_wrapper = RngWrapper::new();
    rng_wrapper.fill_bytes(None);
}

