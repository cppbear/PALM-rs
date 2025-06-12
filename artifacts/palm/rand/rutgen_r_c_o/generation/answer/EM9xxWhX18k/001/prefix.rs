// Answer 0

#[test]
fn test_sample_char_in_surrogate_gap() {
    struct TestRng {
        value: u32,
    }
    
    impl Rng for TestRng {
        fn random<T>(&mut self) -> T {
            // Return maximum random value to hit the edge of the surrogate gap
            unsafe { core::mem::transmute(0xDFFF) }
        }
    }
    
    let mut rng = TestRng { value: 0 };
    let standard_uniform = StandardUniform;
    standard_uniform.sample(&mut rng);
}

#[test]
fn test_sample_char_lower_boundary() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn random<T>(&mut self) -> T {
            // Return a value just below the surrogate gap lower bound
            unsafe { core::mem::transmute(0xD800 - 1) }
        }
    }

    let mut rng = TestRng { value: 0 };
    let standard_uniform = StandardUniform;
    standard_uniform.sample(&mut rng);
}

#[test]
fn test_sample_char_upper_boundary() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn random<T>(&mut self) -> T {
            // Return the maximum valid char value
            unsafe { core::mem::transmute(0x10FFFF) }
        }
    }

    let mut rng = TestRng { value: 0 };
    let standard_uniform = StandardUniform;
    standard_uniform.sample(&mut rng);
}

