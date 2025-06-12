// Answer 0

#[test]
fn test_choose_sample_out_of_bounds() {
    use core::num::NonZeroUsize;

    struct MockRng; // Minimal RNG implementation for testing

    impl Rng for MockRng {
        // Function to simulate sampling behavior
        fn sample_uniform(&mut self, range: UniformUsize) -> usize {
            range.low + range.range // Just return an out-of-bounds index
        }
    }

    let slice: &[i32] = &[1, 2, 3];
    let num_choices = NonZeroUsize::new(1).unwrap();
    let range = UniformUsize {
        low: 0,
        range: slice.len(), // This will be 3, so it will try to return 3, which is out of bounds
        thresh: 0,
        #[cfg(target_pointer_width = "64")]
        mode64: false,
    };
    
    let choose = Choose { slice, range, num_choices };

    let mut rng = MockRng;

    #[should_panic(expected = "Uniform::new(0, 3) somehow returned 3")]
    choose.sample(&mut rng);
}

