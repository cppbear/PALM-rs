// Answer 0

#[test]
fn test_flip_c_heads_all_heads() {
    struct MockRng {
        value: u32,
    }

    impl MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    struct CoinFlipper {
        chunk: u32,
        chunk_remaining: u32,
        rng: MockRng,
    }

    let mut flipper = CoinFlipper {
        chunk: 0b0000_0000_0000_0000_0000_0000_0000_0000, // all zeros for heads
        chunk_remaining: 32,
        rng: MockRng { value: 0 }, // next_u32 will always return 0
    };

    let result = flipper.flip_c_heads(32);
    assert_eq!(result, true);
    assert_eq!(flipper.chunk_remaining, 0);
}

#[test]
fn test_flip_c_heads_with_not_enough_remaining() {
    struct MockRng {
        value: u32,
    }

    impl MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    struct CoinFlipper {
        chunk: u32,
        chunk_remaining: u32,
        rng: MockRng,
    }

    let mut flipper = CoinFlipper {
        chunk: 0b0000_0000_0000_0000_0000_0000_0000_0000, // all zeros for heads
        chunk_remaining: 10, // not enough to consume 32 bits
        rng: MockRng { value: 0 }, // next_u32 will always return 0
    };

    // This will consume the remaining bits and attempt to retrieve a new chunk which will fail to return true.
    let result = flipper.flip_c_heads(32);
    assert_eq!(result, true);
    assert_eq!(flipper.chunk_remaining, 0);
}

