// Answer 0

#[test]
fn test_append_string_zero_length() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct DummyDistribution;

    impl DummyDistribution {
        unsafe fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            (0..).map(move |_| {
                // Generating valid ASCII characters (0-127)
                rng.gen_range(0..=127)
            })
        }
    }

    let mut rng = OsRng;
    let mut string = String::new();
    let len = 0;
    let distribution = DummyDistribution;

    distribution.append_string(&mut rng, &mut string, len);
    
    assert_eq!(string.len(), 0);
}

#[test]
fn test_append_string_non_zero_length() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct DummyDistribution;

    impl DummyDistribution {
        unsafe fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            (0..).map(move |_| {
                // Generating valid ASCII characters (0-127)
                rng.gen_range(0..=127)
            })
        }
    }

    let mut rng = OsRng;
    let mut string = String::new();
    let len = 10;
    let distribution = DummyDistribution;

    distribution.append_string(&mut rng, &mut string, len);
    
    assert_eq!(string.len(), len);
}

#[test]
fn test_append_string_large_length() {
    use rand::rngs::OsRng;
    use rand::Rng;

    struct DummyDistribution;

    impl DummyDistribution {
        unsafe fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            (0..).map(move |_| {
                // Generating valid ASCII characters (0-127)
                rng.gen_range(0..=127)
            })
        }
    }

    let mut rng = OsRng;
    let mut string = String::new();
    let len = 1000; // Test with a large length
    let distribution = DummyDistribution;

    distribution.append_string(&mut rng, &mut string, len);
    
    assert_eq!(string.len(), len);
}

