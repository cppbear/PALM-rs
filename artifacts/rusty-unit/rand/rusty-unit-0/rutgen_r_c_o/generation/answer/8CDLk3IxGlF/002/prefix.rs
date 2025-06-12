// Answer 0

#[test]
fn test_sample_panic_condition() {
    use core::num::NonZeroUsize;
    use crate::distr::uniform::{UniformSampler, UniformUsize};
    struct MockRng;

    impl Rng for MockRng {}

    let slice: &[i32] = &[10]; // single element slice
    let len = slice.len(); // will create a condition to trigger panic
    let range = UniformUsize { low: 0, range: len, thresh: len, mode64: false };
    let num_choices = NonZeroUsize::new(1).unwrap(); // ensures a valid NonZeroUsize
    let chooser = Choose { slice, range, num_choices };

    let mut rng = MockRng;
    chooser.sample(&mut rng); // This should panic as idx will be equal to len
}

#[test]
fn test_sample_valid_input() {
    use core::num::NonZeroUsize;
    use crate::distr::uniform::{UniformSampler, UniformUsize};
    struct MockRng;

    impl Rng for MockRng {}

    let slice: &[i32] = &[1, 2, 3, 4, 5]; // non-empty slice
    let len = slice.len(); 
    let range = UniformUsize { low: 0, range: len, thresh: len, mode64: false };
    let num_choices = NonZeroUsize::new(1).unwrap(); 
    let chooser = Choose { slice, range, num_choices };

    let mut rng = MockRng;
    chooser.sample(&mut rng); // This should not panic and return an element from the slice
}

