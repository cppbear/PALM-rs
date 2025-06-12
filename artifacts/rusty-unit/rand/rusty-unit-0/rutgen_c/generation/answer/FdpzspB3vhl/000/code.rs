// Answer 0

#[cfg(test)]
fn test_sample() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn random<T>(&mut self) -> T {
            self.value as T
        }
    }

    let mut rng = MockRng { value: 5 };
    let standard_uniform = StandardUniform;

    let result = standard_uniform.sample(&mut rng);
    assert_eq!(result, 5u8);
}

#[test]
fn test_sample_boundary_high() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn random<T>(&mut self) -> T {
            self.value as T
        }
    }

    let mut rng = MockRng { value: u8::MAX as u32 };
    let standard_uniform = StandardUniform;

    let result = standard_uniform.sample(&mut rng);
    assert_eq!(result, u8::MAX);
}

#[test]
fn test_sample_boundary_low() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }

        fn random<T>(&mut self) -> T {
            self.value as T
        }
    }

    let mut rng = MockRng { value: 0 };
    let standard_uniform = StandardUniform;

    let result = standard_uniform.sample(&mut rng);
    assert_eq!(result, 0u8);
}

