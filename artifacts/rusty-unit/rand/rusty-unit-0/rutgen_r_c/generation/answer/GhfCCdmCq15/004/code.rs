// Answer 0

#[test]
fn test_append_string_small_slice_and_len() {
    use alloc::string::String;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct TestRng(StdRng);

    impl rand::Rng for TestRng {
        fn gen_range<T>(&mut self, _low: T, _high: T) -> T
        where
            T: rand::distributions::uniform::SampleRange + Copy,
        {
            // Just return the lower bound to avoid unexpected behavior
            _low
        }
    }

    let slice: &[char] = &['a', 'b', 'c', 'd', 'e']; // less than 200 chars
    let num_choices = NonZeroUsize::new(1).unwrap();
    let range = UniformUsize { low: 0, range: 1, thresh: 1, mode64: false };
    let chooser = Choose { slice, range, num_choices };

    let mut rng = TestRng(StdRng::seed_from_u64(0));
    let mut result_string = String::new();
    let len = 50; // less than 100

    chooser.append_string(&mut rng, &mut result_string, len);
    assert_eq!(result_string.len(), len);
}

#[test]
#[should_panic]
fn test_append_string_zero_extend_len() {
    use alloc::string::String;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct TestRng(StdRng);

    impl rand::Rng for TestRng {
        fn gen_range<T>(&mut self, _low: T, _high: T) -> T
        where
            T: rand::distributions::uniform::SampleRange + Copy,
        {
            // Again returning the lower bound
            _low
        }
    }

    let slice: &[char] = &['a', 'b', 'c']; // less than 200 chars
    let num_choices = NonZeroUsize::new(1).unwrap();
    let range = UniformUsize { low: 0, range: 1, thresh: 1, mode64: false };
    let chooser = Choose { slice, range, num_choices };

    let mut rng = TestRng(StdRng::seed_from_u64(0));
    let mut result_string = String::new();
    let len = 200; // this would lead to max_char_len == 1 leading to 0 extend_len

    chooser.append_string(&mut rng, &mut result_string, len);
}

