// Answer 0

#[test]
#[should_panic]
fn test_flip_c_heads_exceeds_limit() {
    struct MockRng;

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0 // mock implementation
        }
    }

    let mut flipper = CoinFlipper::new(MockRng);
    flipper.flip_c_heads(33);
}

