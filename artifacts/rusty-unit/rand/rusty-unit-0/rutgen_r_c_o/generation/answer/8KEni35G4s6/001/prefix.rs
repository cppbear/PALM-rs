// Answer 0

#[test]
fn test_from_rng_valid() {
    struct SimpleRng {
        state: u32,
    }

    impl RngCore for SimpleRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 1;
            self.state
        }

        fn next_u64(&mut self) -> u64 {
            self.state as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }
    }

    let mut rng = SimpleRng { state: 0 };
    let _block_rng: BlockRng<SimpleRng> = BlockRng::from_rng(&mut rng);
}

#[test]
fn test_from_rng_large_rng() {
    struct LargeRng {
        state: u32,
    }

    impl RngCore for LargeRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 2;
            self.state % 4294967295
        }

        fn next_u64(&mut self) -> u64 {
            self.state as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }
    }

    let mut rng = LargeRng { state: 0 };
    let _block_rng: BlockRng<LargeRng> = BlockRng::from_rng(&mut rng);
}

#[test]
fn test_from_rng_zero_length_fill() {
    struct ZeroLengthRng;

    impl RngCore for ZeroLengthRng {
        fn next_u32(&mut self) -> u32 {
            1
        }

        fn next_u64(&mut self) -> u64 {
            1
        }

        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = ZeroLengthRng;
    let _block_rng: BlockRng<ZeroLengthRng> = BlockRng::from_rng(&mut rng);
}

#[test]
fn test_from_rng_boundary_case() {
    struct BoundaryRng {
        state: u32,
    }

    impl RngCore for BoundaryRng {
        fn next_u32(&mut self) -> u32 {
            self.state = 4294967295;
            self.state
        }

        fn next_u64(&mut self) -> u64 {
            self.state as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            let len = dst.len();
            let fill_value = (self.next_u32() % 256) as u8;
            for byte in dst.iter_mut().take(len) {
                *byte = fill_value;
            }
        }
    }

    let mut rng = BoundaryRng { state: 0 };
    let _block_rng: BlockRng<BoundaryRng> = BlockRng::from_rng(&mut rng);
}

