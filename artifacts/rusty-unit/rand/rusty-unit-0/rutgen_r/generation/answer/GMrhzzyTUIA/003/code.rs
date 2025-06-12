// Answer 0

#[test]
fn test_fill_bytes_via_next_full_chunk() {
    struct SimpleRng {
        value: u64,
    }

    impl SimpleRng {
        fn new() -> Self {
            SimpleRng { value: 1 }
        }
    }

    impl rand_core::RngCore for SimpleRng {
        fn next_u32(&mut self) -> u32 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            rand_core::fill_bytes_via_next(self, dest);
        }

        fn from_entropy() -> Self {
            Self::new()
        }
    }

    let mut rng = SimpleRng::new();
    let mut dest = [0u8; 8];
    rand_core::fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [1, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_via_next_partial_chunk_greater_than_4() {
    struct SimpleRng {
        value: u64,
    }

    impl SimpleRng {
        fn new() -> Self {
            SimpleRng { value: 2 }
        }
    }

    impl rand_core::RngCore for SimpleRng {
        fn next_u32(&mut self) -> u32 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            rand_core::fill_bytes_via_next(self, dest);
        }

        fn from_entropy() -> Self {
            Self::new()
        }
    }

    let mut rng = SimpleRng::new();
    let mut dest = [0u8; 5];
    rand_core::fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [2, 0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_via_next_partial_chunk_equal_to_4() {
    struct SimpleRng {
        value: u64,
    }

    impl SimpleRng {
        fn new() -> Self {
            SimpleRng { value: 3 }
        }
    }

    impl rand_core::RngCore for SimpleRng {
        fn next_u32(&mut self) -> u32 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            rand_core::fill_bytes_via_next(self, dest);
        }

        fn from_entropy() -> Self {
            Self::new()
        }
    }

    let mut rng = SimpleRng::new();
    let mut dest = [0u8; 4];
    rand_core::fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [3, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_via_next_zero_length() {
    struct SimpleRng {
        value: u64,
    }

    impl SimpleRng {
        fn new() -> Self {
            SimpleRng { value: 4 }
        }
    }

    impl rand_core::RngCore for SimpleRng {
        fn next_u32(&mut self) -> u32 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value = self.value.wrapping_mul(48271) % 0x7FFFFFFF;
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            rand_core::fill_bytes_via_next(self, dest);
        }

        fn from_entropy() -> Self {
            Self::new()
        }
    }

    let mut rng = SimpleRng::new();
    let mut dest: [u8; 0] = [];
    rand_core::fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest.len(), 0);
}

