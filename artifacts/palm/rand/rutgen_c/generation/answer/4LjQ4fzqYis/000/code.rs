// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRng;

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(84)
        }

        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            for byte in dst.iter_mut() {
                *byte = 255; // Fill with a constant value for testing
            }
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut buffer = [0u8; 10];

    let mut unwrap_err_rng = UnwrapErr(rng);
    unwrap_err_rng.fill_bytes(&mut buffer);

    assert_eq!(buffer, [255; 10]);
}

