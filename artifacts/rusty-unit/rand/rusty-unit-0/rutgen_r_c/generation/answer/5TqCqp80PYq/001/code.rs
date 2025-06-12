// Answer 0

#[test]
fn test_fill_bytes_success() {
    struct MockRng {
        fill_bytes_called: bool,
    }

    impl TryRngCore for MockRng {
        type Error = &'static str;

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(84)
        }

        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            self.fill_bytes_called = true;
            if dst.len() == 0 {
                Err("Cannot fill zero-length slice")
            } else {
                for i in 0..dst.len() {
                    dst[i] = i as u8;
                }
                Ok(())
            }
        }
    }

    let mut mock_rng = MockRng { fill_bytes_called: false };
    let mut dst = [0u8; 10];

    let mut unwrap_mut = UnwrapMut(&mut mock_rng);
    unwrap_mut.fill_bytes(&mut dst);

    assert!(mock_rng.fill_bytes_called);
    assert_eq!(dst, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
#[should_panic(expected = "Cannot fill zero-length slice")]
fn test_fill_bytes_zero_length() {
    struct MockRng {}

    impl TryRngCore for MockRng {
        type Error = &'static str;

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(0)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(0)
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Err("Cannot fill zero-length slice")
        }
    }

    let mut mock_rng = MockRng {};
    let mut dst: [u8; 0] = [];
    
    let mut unwrap_mut = UnwrapMut(&mut mock_rng);
    unwrap_mut.fill_bytes(&mut dst);
}

