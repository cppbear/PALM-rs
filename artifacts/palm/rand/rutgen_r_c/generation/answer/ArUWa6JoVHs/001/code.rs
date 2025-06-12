// Answer 0

#[test]
fn test_fill_u16() {
    struct MockRng {
        counter: u16,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.counter as u8; // Produces predictable bytes for testing
                self.counter = self.counter.wrapping_add(1);
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut array: [u16; 5] = [0; 5];
    let mut rng = MockRng { counter: 1 };
    array.fill(&mut rng);

    assert_eq!(array, [1, 2, 3, 4, 5]);
}

#[test]
fn test_fill_i32() {
    struct MockRng {
        counter: i32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.counter as u8; // Produces predictable bytes for testing
                self.counter = self.counter.wrapping_add(1);
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut array: [i32; 3] = [0; 3];
    let mut rng = MockRng { counter: 1 };
    array.fill(&mut rng);

    assert_eq!(array, [1, 2, 3]);
}

#[test]
fn test_fill_bool() {
    struct MockRng {
        true_count: u8,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.true_count as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.true_count as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = if self.true_count % 2 == 0 { 1 } else { 0 }; // Alternates between true and false
                self.true_count = self.true_count.wrapping_add(1);
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut array: [bool; 4] = [false; 4];
    let mut rng = MockRng { true_count: 0 };
    array.fill(&mut rng);

    assert_eq!(array, [true, false, true, false]);
}

#[test]
fn test_fill_char() {
    struct MockRng {
        counter: char,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.counter as u8; // Produces predictable bytes for testing
                self.counter = char::from_u32((self.counter as u32 + 1) % 128).unwrap(); // Wraps around the ASCII range
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut array: [char; 3] = ['a'; 3];
    let mut rng = MockRng { counter: 'a' };
    array.fill(&mut rng);

    assert_eq!(array, ['a', 'b', 'c']);
}

