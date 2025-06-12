// Answer 0

#[test]
fn test_sample_valid_char() {
    use rand::Rng;
    use rand::distributions::Uniform;
    use rand::thread_rng;

    struct CharSampler;

    impl CharSampler {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            const GAP_SIZE: u32 = 0xDFFF - 0xD800 + 1;
            let range = Uniform::new(GAP_SIZE, 0x11_0000).unwrap();
            let mut n = range.sample(rng);
            if n <= 0xDFFF {
                n -= GAP_SIZE;
            }
            unsafe { char::from_u32_unchecked(n) }
        }
    }

    let sampler = CharSampler;
    let mut rng = thread_rng();

    for _ in 0..1000 {
        let c = sampler.sample(&mut rng);
        assert!((c as u32) < 0xD800 || (c as u32) > 0xDFFF, "The character '{}' is in the surrogate range.", c);
    }
}

#[test]
#[should_panic]
fn test_sample_uniform_panic() {
    use rand::Rng;
    use rand::distributions::Uniform;

    struct CharSampler;

    impl CharSampler {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            const GAP_SIZE: u32 = 0xDFFF - 0xD800 + 1;
            let range = Uniform::new(GAP_SIZE, 0x11_0000).unwrap();
            let mut n = range.sample(rng);
            if n <= 0xDFFF {
                n -= GAP_SIZE;
            }
            unsafe { char::from_u32_unchecked(n) }
        }
    }

    let sampler = CharSampler;
    let mut rng = rand::thread_rng();

    // Focusing on edge cases where Uniform might panic
    let _ = Uniform::new(0, 0).unwrap();  // This should trigger a panic when unwrapping
}

