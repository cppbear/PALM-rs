// Answer 0

#[test]
fn test_flip_c_heads_case_1() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0 // simulate a case where all bits are zeros, ensuring leading_zeros returns 32
        }
    }

    let mut flipper = CoinFlipper::new(MockRng {});

    flipper.chunk_remaining = 33; // Ensure chunk_remaining > c
    flipper.chunk = 0; // Set chunk to 0 to simulate zeros

    let result = flipper.flip_c_heads(32);
}

#[test]
fn test_flip_c_heads_case_2() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0 // simulate a case where all bits are zeros
        }
    }

    let mut flipper = CoinFlipper::new(MockRng {});

    flipper.chunk_remaining = 33; // Ensure chunk_remaining > c
    flipper.chunk = 0; // Set chunk to 0 to simulate leading_zeros equal to c

    let result = flipper.flip_c_heads(32);
}

#[test]
fn test_flip_c_heads_case_3() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0b00000000000000000000000000000000 // All zeros, leading_zeros will be 32
        }
    }

    let mut flipper = CoinFlipper::new(MockRng {});

    flipper.chunk_remaining = 33; // Ensure chunk_remaining > c
    flipper.chunk = 0; // Set chunk to all zeros

    let result = flipper.flip_c_heads(32);
}

