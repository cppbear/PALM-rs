// Answer 0

#[test]
fn test_flip_c_heads_case1() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0b11111111111111111111111111111111 // all bits are heads
        }
    }

    let mut flipper = CoinFlipper::new(TestRng);
    flipper.chunk = 0; // set chunk to all heads
    flipper.chunk_remaining = 32; // maximum remaining bits
    let result = flipper.flip_c_heads(32);
}

#[test]
fn test_flip_c_heads_case2() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0b01111111111111111111111111111111 // first bit is a tail
        }
    }

    let mut flipper = CoinFlipper::new(TestRng);
    flipper.chunk = 0b11111111111111111111111111111111; // set chunk to all heads initially
    flipper.chunk_remaining = 32; // maximum remaining bits
    let result = flipper.flip_c_heads(32);
}

