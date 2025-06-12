// Answer 0

#[cfg(test)]
mod tests {
    use crate::seq::coin_flipper::CoinFlipper;
    use crate::RngCore;
    use rand::rngs::MockRng;

    #[test]
    fn test_flip_c_heads_all_heads() {
        struct TestRng {
            values: Vec<u32>,
            index: usize,
        }

        impl RngCore for TestRng {
            fn next_u32(&mut self) -> u32 {
                let value = self.values[self.index];
                self.index += 1;
                value
            }

            fn next_u64(&mut self) -> u64 {
                unimplemented!()
            }

            fn fill_bytes(&mut self, dest: &mut [u8]) {
                unimplemented!()
            }

            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
                unimplemented!()
            }
        }

        let rng = TestRng {
            values: vec![0b00000000_00000000_00000000_00000000],
            index: 0,
        };
        let mut coin_flipper = CoinFlipper::new(rng);
        assert_eq!(coin_flipper.flip_c_heads(32), true);
    }

    #[test]
    #[should_panic]
    fn test_flip_c_heads_boundary() {
        let rng = TestRng {
            values: vec![0b00000000_00000000_00000000_00000001],
            index: 0,
        };
        let mut coin_flipper = CoinFlipper::new(rng);
        coin_flipper.flip_c_heads(33); // Should panic due to invalid input
    }

    #[test]
    fn test_flip_c_heads_not_all_heads() {
        struct TestRng {
            state: u32,
        }

        impl RngCore for TestRng {
            fn next_u32(&mut self) -> u32 {
                self.state
            }

            fn next_u64(&mut self) -> u64 {
                unimplemented!()
            }

            fn fill_bytes(&mut self, dest: &mut [u8]) {
                unimplemented!()
            }

            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
                unimplemented!()
            }
        }

        let rng = TestRng { state: 0b00000000_00000000_00000000_00000001 };
        let mut coin_flipper = CoinFlipper::new(rng);
        assert_eq!(coin_flipper.flip_c_heads(1), false);
    }
}

