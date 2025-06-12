// Answer 0

#[test]
fn test_coin_flipper_new() {
    struct MockRng;

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0 // mock implementation
        }
        fn next_u64(&mut self) -> u64 {
            0 // mock implementation
        }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            // mock implementation
        }
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), ()> {
            Ok(()) // mock implementation
        }
    }

    let mock_rng = MockRng;
    let coin_flipper = CoinFlipper::new(mock_rng);

    assert_eq!(coin_flipper.chunk, 0);
    assert_eq!(coin_flipper.chunk_remaining, 0);
}

