// Answer 0

#[test]
fn test_sample_non_panicking() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
        
        fn random<T: rand::distributions::Distribution<u8>>(&mut self) -> u8 {
            self.next_u32() as u8
        }
    }

    let mut rng = MockRng { value: 0 };
    let standard_uniform = StandardUniform;
    assert_eq!(standard_uniform.sample(&mut rng), 0);

    rng.value = 255;
    assert_eq!(standard_uniform.sample(&mut rng), 255);

    rng.value = 128;
    assert_eq!(standard_uniform.sample(&mut rng), 128);

    rng.value = 1;
    assert_eq!(standard_uniform.sample(&mut rng), 1);
}

#[test]
#[should_panic(expected = "some expected panic condition")]
fn test_sample_panic() {
    struct PanicRng;

    impl Rng for PanicRng {
        fn next_u32(&mut self) -> u32 {
            panic!("Intentional panic for testing")
        }
        
        fn random<T: rand::distributions::Distribution<u8>>(&mut self) -> u8 {
            self.next_u32() as u8
        }
    }

    let mut rng = PanicRng;
    let standard_uniform = StandardUniform;

    let _ = standard_uniform.sample(&mut rng);
}

