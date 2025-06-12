// Answer 0

#[test]
fn test_try_from_rng_success_small_rng() {
    struct SmallRng {
        state: u64,
    }

    impl RngCore for SmallRng {
        fn next_u32(&mut self) -> u32 {
            self.state = self.state.wrapping_add(1);
            self.state as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(1);
            self.state
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.state & 0xFF) as u8;
                self.state = self.state.wrapping_add(1);
            }
        }
    }

    impl TryRngCore for SmallRng {
        type Error = ();
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            self.fill_bytes(dst);
            Ok(())
        }
    }

    let mut rng = SmallRng { state: 0 };
    let result: BlockRng64<SmallRng> = BlockRng64::try_from_rng(&mut rng).unwrap();
}

#[test]
fn test_try_from_rng_success_large_rng() {
    struct LargeRng {
        state: u64,
    }

    impl RngCore for LargeRng {
        fn next_u32(&mut self) -> u32 {
            self.state = self.state.wrapping_add(1);
            self.state as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(1);
            self.state
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.state & 0xFF) as u8;
                self.state = self.state.wrapping_add(1);
            }
        }
    }

    impl TryRngCore for LargeRng {
        type Error = ();
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            self.fill_bytes(dst);
            Ok(())
        }
    }

    let mut rng = LargeRng { state: 1024 };
    let result: BlockRng64<LargeRng> = BlockRng64::try_from_rng(&mut rng).unwrap();
}

#[test]
#[should_panic]
fn test_try_from_rng_fail_try_fill_bytes() {
    struct FailingRng;

    impl RngCore for FailingRng {
        fn next_u32(&mut self) -> u32 {
            0
        }

        fn next_u64(&mut self) -> u64 {
            0
        }

        fn fill_bytes(&mut self, _: &mut [u8]) {
        }
    }

    impl TryRngCore for FailingRng {
        type Error = ();
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Err(())
        }
    }

    let mut rng = FailingRng;
    let _result: BlockRng64<FailingRng> = BlockRng64::try_from_rng(&mut rng).unwrap();
}

#[test]
fn test_try_from_rng_success_edge_state() {
    struct EdgeRng {
        state: u64,
    }

    impl RngCore for EdgeRng {
        fn next_u32(&mut self) -> u32 {
            self.state = self.state.wrapping_add(1);
            self.state as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(1);
            self.state
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.state & 0xFF) as u8;
                self.state = self.state.wrapping_add(1);
            }
        }
    }

    impl TryRngCore for EdgeRng {
        type Error = ();
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            self.fill_bytes(dst);
            Ok(())
        }
    }

    let mut rng = EdgeRng { state: 18446744073709551615 };
    let result: BlockRng64<EdgeRng> = BlockRng64::try_from_rng(&mut rng).unwrap();
}

