// Answer 0

#[cfg(test)]
fn test_sample() {
    struct MockRng {
        value: u8,
        calls: usize,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.calls += 1;
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.calls += 1;
            self.value as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.value;
            }
            self.calls += 1;
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    struct SampledUniform; 

    impl SampleUniform for SampledUniform {
        type Sampler = Self;

        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<u8, rand::Error> {
            Ok(rng.next_u32() as u8)
        }
    }

    let sampler = SampledUniform {};
    let uniform = Uniform(sampler);

    // Testing normal range
    {
        let mut rng = MockRng { value: 10, calls: 0 };
        let result = uniform.sample(&mut rng);
        assert_eq!(result.unwrap(), 10);
        assert_eq!(rng.calls, 1);
    }

    // Testing edge condition with 0
    {
        let mut rng = MockRng { value: 0, calls: 0 };
        let result = uniform.sample(&mut rng);
        assert_eq!(result.unwrap(), 0);
        assert_eq!(rng.calls, 1);
    }

    // Testing with max value of u8
    {
        let mut rng = MockRng { value: 255, calls: 0 };
        let result = uniform.sample(&mut rng);
        assert_eq!(result.unwrap(), 255);
        assert_eq!(rng.calls, 1);
    }
}

