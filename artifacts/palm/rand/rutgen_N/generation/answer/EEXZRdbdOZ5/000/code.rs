// Answer 0

#[test]
fn test_sample() {
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    struct SampleStruct;

    impl SampleStruct {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
            const RANGE: u8 = 26 + 26;

            let offset = rng.gen_range(0..RANGE) + b'A';

            offset + (offset > b'Z') as u8 * (b'a' - b'Z' - 1)
        }
    }

    // Test case 1: Check a regular sample
    let mut rng = StdRng::seed_from_u64(0);
    let sample_struct = SampleStruct;
    let result = sample_struct.sample(&mut rng);
    assert!(result >= b'A' && result <= b'z');

    // Test case 2: Seed provides a predictable outcome
    rng = StdRng::seed_from_u64(0);
    let result2 = sample_struct.sample(&mut rng);
    assert_eq!(result, result2);

    // Test case 3: Check boundary values
    rng = StdRng::seed_from_u64(1);
    let boundary_result = sample_struct.sample(&mut rng);
    assert!(boundary_result >= b'A' && boundary_result <= b'z');
}

