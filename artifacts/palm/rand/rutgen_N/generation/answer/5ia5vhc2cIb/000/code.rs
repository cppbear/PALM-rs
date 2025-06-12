// Answer 0

#[test]
fn test_append_string() {
    use rand::Rng;
    use rand::thread_rng;

    struct MockSampler;

    impl MockSampler {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = char> {
            let characters = vec!['a', '1', 'b', '2'];
            std::iter::from_fn(move || {
                let index = rng.gen_range(0..characters.len());
                Some(characters[index])
            })
        }
    }

    let mut rng = thread_rng();
    let mut string = String::new();
    let sampler = MockSampler;

    sampler.append_string(&mut rng, &mut string, 10);
    
    assert_eq!(string.len(), 10);
    for c in string.chars() {
        assert!(c.is_ascii_alphanumeric());
    }
}

#[test]
fn test_append_string_with_zero_length() {
    use rand::Rng;
    use rand::thread_rng;

    struct MockSampler;

    impl MockSampler {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = char> {
            let characters = vec!['a', '1', 'b', '2'];
            std::iter::from_fn(move || {
                let index = rng.gen_range(0..characters.len());
                Some(characters[index])
            })
        }
    }

    let mut rng = thread_rng();
    let mut string = String::new();
    let sampler = MockSampler;

    sampler.append_string(&mut rng, &mut string, 0);
    
    assert_eq!(string.len(), 0);
}

