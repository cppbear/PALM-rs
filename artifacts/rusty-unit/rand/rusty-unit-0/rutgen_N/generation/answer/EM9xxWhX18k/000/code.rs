// Answer 0

#[test]
fn test_sample_valid_char() {
    use rand::Rng;
    use rand::thread_rng;
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

    let mut rng = thread_rng();
    let sampler = CharSampler;
    let sampled_char = sampler.sample(&mut rng);
    assert!(sampled_char as u32 < 0xD800 || sampled_char as u32 > 0xDFFF);
    assert!(sampled_char as u32 < 0x110000);
}

#[test]
fn test_sample_boundary_chars() {
    use rand::Rng;
    use rand::thread_rng;
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

    let mut rng = thread_rng();
    let sampler = CharSampler;

    for _ in 0..100 {
        let sampled_char = sampler.sample(&mut rng);
        assert!(sampled_char as u32 < 0xD800 || sampled_char as u32 > 0xDFFF);
        assert!(sampled_char as u32 < 0x110000);
    }
}

