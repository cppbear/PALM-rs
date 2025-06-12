// Answer 0

#[test]
fn test_sample_array_n_greater_than_len() {
    struct MockRng;

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            range.start // This mock function always returns the start of the range
        }
    }

    let mut rng = MockRng;

    let len_0 = 0;
    let n_1 = 1;
    let result_1 = sample_array::<MockRng, 1>(&mut rng, len_0);

    let len_50 = 50;
    let n_51 = 51;
    let result_2 = sample_array::<MockRng, 51>(&mut rng, len_50);

    let len_99 = 99;
    let n_100 = 100;
    let result_3 = sample_array::<MockRng, 100>(&mut rng, len_99);
}

