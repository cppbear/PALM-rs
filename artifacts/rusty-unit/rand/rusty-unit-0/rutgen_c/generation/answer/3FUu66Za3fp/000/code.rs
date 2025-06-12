// Answer 0

#[test]
fn test_next_u64_success() {
    struct TestRng;

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(84)
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut unwrap_mut = UnwrapMut(&mut rng);
    let result = unwrap_mut.next_u64();
    assert_eq!(result, 84);
}

#[test]
#[should_panic]
fn test_next_u64_failure() {
    struct FailingRng;

    impl TryRngCore for FailingRng {
        type Error = &'static str;

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Err("Error")
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Err("Error")
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = FailingRng;
    let mut unwrap_mut = UnwrapMut(&mut rng);
    let _result = unwrap_mut.next_u64(); // This will panic
}

