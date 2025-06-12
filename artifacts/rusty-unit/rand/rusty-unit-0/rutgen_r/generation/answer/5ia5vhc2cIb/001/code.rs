// Answer 0

#[test]
fn test_append_string_with_valid_rng_and_string() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct AlphanumericSampler;

    impl AlphanumericSampler {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            std::iter::repeat_with(move || chars[rng.gen_range(0..chars.len())])
        }
    }

    let mut rng = OsRng;
    let mut result_string = String::new();
    let sampler = AlphanumericSampler;

    sampler.append_string(&mut rng, &mut result_string, 10);
    assert_eq!(result_string.len(), 10);
    assert!(result_string.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_append_string_with_zero_length() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct AlphanumericSampler;

    impl AlphanumericSampler {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            std::iter::repeat_with(move || chars[rng.gen_range(0..chars.len())])
        }
    }

    let mut rng = OsRng;
    let mut result_string = String::from("Initial");
    let sampler = AlphanumericSampler;

    sampler.append_string(&mut rng, &mut result_string, 0);
    assert_eq!(result_string, "Initial");
}

#[test]
#[should_panic]
fn test_append_string_with_negative_length() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct AlphanumericSampler;

    impl AlphanumericSampler {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            std::iter::repeat_with(move || chars[rng.gen_range(0..chars.len())])
        }
    }

    let mut rng = OsRng;
    let mut result_string = String::new();
    let sampler = AlphanumericSampler;

    sampler.append_string(&mut rng, &mut result_string, usize::MAX);
}

#[test]
fn test_append_string_with_large_length() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct AlphanumericSampler;

    impl AlphanumericSampler {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            std::iter::repeat_with(move || chars[rng.gen_range(0..chars.len())])
        }
    }

    let mut rng = OsRng;
    let mut result_string = String::new();
    let sampler = AlphanumericSampler;

    let length = 1000; // Testing with a large length
    sampler.append_string(&mut rng, &mut result_string, length);
    assert_eq!(result_string.len(), length);
    assert!(result_string.chars().all(|c| c.is_ascii_alphanumeric()));
}

