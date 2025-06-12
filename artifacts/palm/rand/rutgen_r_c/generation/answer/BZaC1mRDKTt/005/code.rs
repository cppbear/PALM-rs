// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;
    use rand::RngCore;

    struct MockRng {
        value: u32,
        next_value: Option<u32>,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.next_value.take().unwrap_or(self.value)
        }

        fn next_u64(&mut self) -> u64 {
            (self.next_u32() as u64) | ((self.next_u32() as u64) << 32)
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
            unimplemented!()
        }
    }

    #[test]
    fn test_random_ratio_case_1() {
        let mut rng = MockRng {
            value: 0b00000000_00000000_00000000_00000001, // This will make flip_c_heads return false
            next_value: None,
        };
        let mut coin_flipper = CoinFlipper::new(rng);
        
        // These values satisfy the condition n < d
        let n = 3; // example value for n
        let d = 10; // example value for d

        let result = coin_flipper.random_ratio(n, d);
        assert_eq!(result, false);
    }
}

