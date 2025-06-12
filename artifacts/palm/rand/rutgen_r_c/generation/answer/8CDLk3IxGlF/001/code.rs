// Answer 0

#[test]
fn test_sample_valid_index() {
    use core::num::NonZeroUsize;
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn gen_range(&mut self, low: usize, high: usize) -> usize {
            // Always return a valid index in the range
            self.current % (high - low) + low
        }
        
        // Other methods can remain unimplemented for the mock
        fn gen_bool(&mut self, _: f64) -> bool { false }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
        // Additional methods can be added as needed
    }

    let slice = &[10, 20, 30, 40, 50];
    let num_choices = NonZeroUsize::new(slice.len()).unwrap();
    let range = UniformUsize { low: 0, range: slice.len(), thresh: 0, mode64: false };
    
    let choose = Choose { slice, range, num_choices };
    let mut rng = MockRng { current: 0 };

    for _ in 0..10 {
        let sample = choose.sample(&mut rng);
        assert!(slice.contains(sample)); // Ensures the sample is from the slice
    }
}

#[test]
#[should_panic(expected = "Uniform::new(0, 0)")]
fn test_sample_empty_slice_panics() {
    use core::num::NonZeroUsize;
    use rand::Rng;

    let slice: &[u32] = &[];
    let num_choices = NonZeroUsize::new(0).unwrap();
    
    let range = UniformUsize { low: 0, range: 0, thresh: 0, mode64: false };
    
    let choose = Choose { slice, range, num_choices };
    let mut rng = rand::thread_rng();
    
    choose.sample(&mut rng); // Should panic due to empty slice
}

#[test]
fn test_sample_with_full_range() {
    use core::num::NonZeroUsize;
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let slice = &[1, 2, 3, 4, 5];
    let num_choices = NonZeroUsize::new(slice.len()).unwrap();
    let range = UniformUsize { low: 0, range: slice.len(), thresh: 0, mode64: false };

    let choose = Choose { slice, range, num_choices };
    let mut rng = StdRng::from_seed([0; 32]); // Use a seeded RNG for deterministic results

    let samples: Vec<_> = (0..10).map(|_| choose.sample(&mut rng)).collect();
    samples.iter().for_each(|&s| {
        assert!(slice.contains(s)); // Ensures the sample is from the slice
    });
}

