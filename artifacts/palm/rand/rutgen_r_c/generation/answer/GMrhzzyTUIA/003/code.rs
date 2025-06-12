// Answer 0

fn fill_bytes_via_next_test() {
    struct TestRng {
        counter: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.counter as u32;
            self.counter += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            let value = self.counter;
            self.counter += 1;
            value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }

        fn throw_unimplemented(&mut self) {
            panic!("not implemented");
        }
    }

    // Test case for left.len() == 8 and ensures proper functionality
    {
        let mut rng = TestRng { counter: 1 };
        let mut buffer = [0u8; 8];
        fill_bytes_via_next(&mut rng, &mut buffer);
        assert_eq!(buffer, (1u64.to_le_bytes())); // Expecting the first call to next_u64
    }

    // Test case for left.len() < 8, specifically n == 0
    {
        let mut rng = TestRng { counter: 1 };
        let mut buffer = [0u8; 0]; // n == 0
        fill_bytes_via_next(&mut rng, &mut buffer);
        assert_eq!(buffer, []); // Buffer should remain unchanged
    }

    // Test case for left.len() < 8, specifically n == 4
    {
        let mut rng = TestRng { counter: 1 };
        let mut buffer = [0u8; 4]; // n == 4
        fill_bytes_via_next(&mut rng, &mut buffer);
        assert_eq!(buffer, (1u64.to_le_bytes()[..4].to_vec()).as_slice()); // Expecting first 4 bytes from first u64
    }

    // Test case to ensure the function behaves correctly when left.len() > 8 primarily
    {
        let mut rng = TestRng { counter: 8 }; // starting with 8 to see the behavior
        let mut buffer = [0u8; 16]; // Will fill with 2 u64 values
        fill_bytes_via_next(&mut rng, &mut buffer);
        assert_eq!(buffer[..8], (8u64.to_le_bytes())); // Expecting first u64
        assert_eq!(buffer[8..16], (9u64.to_le_bytes())); // Expecting second u64
    }
}

