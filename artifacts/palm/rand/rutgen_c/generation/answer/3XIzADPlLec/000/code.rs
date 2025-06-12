// Answer 0

#[test]
fn test_sample_uniform_u8() {
    struct MockRng {
        value: u8,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }
        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(self.value);
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    struct UniformU8Sampler;

    impl UniformSampler for UniformU8Sampler {
        type X = u8;

        fn sample<R: RngCore + ?Sized>(&self, rng: &mut R) -> u8 {
            rng.next_u32() as u8
        }
    }

    struct UniformU8(UniformU8Sampler);

    impl SampleUniform for u8 {
        type Sampler = UniformU8Sampler;
    }

    let uniform_u8 = Uniform(UniformU8Sampler);
    let mut rng = MockRng { value: 42 };

    let result = uniform_u8.sample(&mut rng);
    assert_eq!(result, 42);
}

#[test]
fn test_sample_uniform_u32() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
        fn next_u64(&mut self) -> u64 {
            self.value as u64
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(self.value as u8);
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    struct UniformU32Sampler;

    impl UniformSampler for UniformU32Sampler {
        type X = u32;

        fn sample<R: RngCore + ?Sized>(&self, rng: &mut R) -> u32 {
            rng.next_u32()
        }
    }

    struct UniformU32(UniformU32Sampler);

    impl SampleUniform for u32 {
        type Sampler = UniformU32Sampler;
    }

    let uniform_u32 = Uniform(UniformU32Sampler);
    let mut rng = MockRng { value: 12345 };

    let result = uniform_u32.sample(&mut rng);
    assert_eq!(result, 12345);
}

#[test]
fn test_sample_uniform_u64() {
    struct MockRng {
        value: u64,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }
        fn next_u64(&mut self) -> u64 {
            self.value
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(self.value as u8);
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    struct UniformU64Sampler;

    impl UniformSampler for UniformU64Sampler {
        type X = u64;

        fn sample<R: RngCore + ?Sized>(&self, rng: &mut R) -> u64 {
            rng.next_u64()
        }
    }

    struct UniformU64(UniformU64Sampler);

    impl SampleUniform for u64 {
        type Sampler = UniformU64Sampler;
    }

    let uniform_u64 = Uniform(UniformU64Sampler);
    let mut rng = MockRng { value: 9876543210 };

    let result = uniform_u64.sample(&mut rng);
    assert_eq!(result, 9876543210);
}

