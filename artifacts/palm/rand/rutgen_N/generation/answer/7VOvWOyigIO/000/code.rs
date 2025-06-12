// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;
    use rand::thread_rng;

    struct DummySampler;

    impl DummySampler {
        fn sample_single(start: i32, end: i32, rng: &mut dyn RngCore) -> Result<i32, &'static str> {
            if start >= end {
                return Err("invalid range");
            }
            let range = end - start;
            let value = rng.next_u32() % range as u32;
            Ok(start + value as i32)
        }
    }

    struct DummyRng;

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            42 // return a constant value for predictable testing
        }

        fn next_u64(&mut self) -> u64 {
            42 // similarly predictable
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 42; // fill with constants
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    #[test]
    fn test_sample_single_valid_range() {
        let mut rng = DummyRng;
        let result = DummySampler::sample_single(2, 5, &mut rng);
        assert!(result.is_ok());
        assert!(result.unwrap() >= 2 && result.unwrap() < 5);
    }

    #[test]
    #[should_panic(expected = "invalid range")]
    fn test_sample_single_invalid_range() {
        let mut rng = DummyRng;
        DummySampler::sample_single(5, 5, &mut rng).unwrap(); // This should panic.
    }
}

