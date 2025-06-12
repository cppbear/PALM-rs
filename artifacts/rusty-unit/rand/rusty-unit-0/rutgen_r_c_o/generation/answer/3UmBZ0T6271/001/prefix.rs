// Answer 0

#[test]
fn test_next_u64_simple_success() {
    struct TestRng;

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(1234567890)
        }

        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = UnwrapErr(TestRng);
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_minimum_value() {
    struct MinRng;

    impl TryRngCore for MinRng {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(0)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(0)
        }

        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = UnwrapErr(MinRng);
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_maximum_value() {
    struct MaxRng;

    impl TryRngCore for MaxRng {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(u32::MAX)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(u64::MAX)
        }

        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = UnwrapErr(MaxRng);
    let _result = rng.next_u64();
}

#[test]
#[should_panic]
fn test_next_u64_failure() {
    struct FailingRng;

    impl TryRngCore for FailingRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Err(())
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Err(())
        }

        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = UnwrapErr(FailingRng);
    let _result = rng.next_u64();
}

