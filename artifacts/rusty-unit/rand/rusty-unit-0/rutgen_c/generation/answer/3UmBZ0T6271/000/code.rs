// Answer 0

#[test]
fn test_next_u64_valid() {
    struct MockRng;

    impl TryRngCore for MockRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(84)
        }

        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            dst.copy_from_slice(&[1, 2, 3, 4]);
            Ok(())
        }
    }

    let mut mock_rng = MockRng;
    let unwrap_err_rng = UnwrapErr(mock_rng);
    assert_eq!(unwrap_err_rng.next_u64(), 84);
}

#[test]
#[should_panic]
fn test_next_u64_error() {
    struct MockErrorRng;

    impl TryRngCore for MockErrorRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Err(())
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Err(())
        }

        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            Err(())
        }
    }

    let mut mock_rng = MockErrorRng;
    let unwrap_err_rng = UnwrapErr(mock_rng);
    // This should panic because it expects to unwrap a Result that is an error
    let _ = unwrap_err_rng.next_u64();
}

