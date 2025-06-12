// Answer 0

#[test]
fn test_sample_valid() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use core::num::NonZeroUsize;

    let mut rng = StdRng::seed_from_u64(0);
    let slice = &[1, 2, 3, 4, 5];
    let range = UniformUsize { low: 0, range: 5, thresh: 0, mode64: false };
    let num_choices = NonZeroUsize::new(1).unwrap();
    let chooser = Choose { slice, range, num_choices };

    let result = chooser.sample(&mut rng);
}

#[test]
fn test_sample_single_element() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use core::num::NonZeroUsize;

    let mut rng = StdRng::seed_from_u64(1);
    let slice = &[42];
    let range = UniformUsize { low: 0, range: 1, thresh: 0, mode64: false };
    let num_choices = NonZeroUsize::new(1).unwrap();
    let chooser = Choose { slice, range, num_choices };

    let result = chooser.sample(&mut rng);
}

#[test]
fn test_sample_multiple_elements() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use core::num::NonZeroUsize;

    let mut rng = StdRng::seed_from_u64(2);
    let slice = &["a", "b", "c", "d", "e"];
    let range = UniformUsize { low: 0, range: 5, thresh: 0, mode64: false };
    let num_choices = NonZeroUsize::new(1).unwrap();
    let chooser = Choose { slice, range, num_choices };

    let result = chooser.sample(&mut rng);
}

#[test]
fn test_sample_empty_slice() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use core::num::NonZeroUsize;

    let mut rng = StdRng::seed_from_u64(3);
    let slice: &[i32] = &[];
    let range = UniformUsize { low: 0, range: 0, thresh: 0, mode64: false };
    let num_choices = NonZeroUsize::new(1).unwrap();
    let chooser = Choose { slice, range, num_choices };

    let result = chooser.sample(&mut rng);
}

#[test]
#[should_panic]
fn test_sample_index_out_of_bounds() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use core::num::NonZeroUsize;

    let mut rng = StdRng::seed_from_u64(4);
    let slice = &[1, 2, 3];
    let range = UniformUsize { low: 0, range: 5, thresh: 0, mode64: false }; // This will create an index out of bounds
    let num_choices = NonZeroUsize::new(1).unwrap();
    let chooser = Choose { slice, range, num_choices };

    let result = chooser.sample(&mut rng);
}

