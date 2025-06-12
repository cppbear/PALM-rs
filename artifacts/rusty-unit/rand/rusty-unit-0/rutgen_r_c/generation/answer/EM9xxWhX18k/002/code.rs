// Answer 0

#[test]
fn test_sample_char_valid() {
    struct DummyRng;

    impl Rng for DummyRng {
        fn random<T>(&mut self) -> T where StandardUniform: Distribution<T>, {
            // Choosing a value greater than the surrogate gap and within valid `char` range
            // The value must be between (0xDFFF, 0x110000)
            if std::mem::size_of::<T>() == 4 {
                return unsafe { std::mem::transmute(0xE000u32) }; // Example character
            }
            panic!("This test only supports u32 types");
        }
    }

    let standard_uniform = StandardUniform;
    let mut rng = DummyRng;

    // Call the sample method and check if it returns a valid `char`.
    let result = standard_uniform.sample(&mut rng);
    assert!(result >= '\u{E000}' && result <= '\u{10FFFF}');
}

#[test]
#[should_panic]
fn test_sample_char_surrogate() {
    struct PanicRng;

    impl Rng for PanicRng {
        fn random<T>(&mut self) -> T where StandardUniform: Distribution<T>, {
            // Choosing a value within the surrogate gap
            if std::mem::size_of::<T>() == 4 {
                return unsafe { std::mem::transmute(0xD800u32) }; // Example surrogate character
            }
            panic!("This test only supports u32 types");
        }
    }

    let standard_uniform = StandardUniform;
    let mut rng = PanicRng;

    // This should panic due to wanting to generate a surrogate character
    let _ = standard_uniform.sample(&mut rng);
}

#[test]
fn test_sample_char_boundary() {
    struct EdgeRng;

    impl Rng for EdgeRng {
        fn random<T>(&mut self) -> T where StandardUniform: Distribution<T>, {
            // Choosing an edge value slightly above the surrogate gap
            if std::mem::size_of::<T>() == 4 {
                return unsafe { std::mem::transmute(0xD800u32 + 1) }; // First valid character after surrogate
            }
            panic!("This test only supports u32 types");
        }
    }

    let standard_uniform = StandardUniform;
    let mut rng = EdgeRng;

    // Call the sample method
    let result = standard_uniform.sample(&mut rng);
    assert!(result >= '\u{E000}' && result <= '\u{10FFFF}');
}

