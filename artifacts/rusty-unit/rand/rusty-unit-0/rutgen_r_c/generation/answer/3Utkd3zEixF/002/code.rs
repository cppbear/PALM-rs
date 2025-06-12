// Answer 0

#[test]
fn test_increasing_uniform_new_zero() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0 // deterministic output for consistent tests
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), ()> {
            Ok(())
        }
    }

    let rng = TestRng;
    let n = 0;
    let increasing_uniform = IncreasingUniform::new(rng, n);

    assert_eq!(increasing_uniform.n, 0);
    assert_eq!(increasing_uniform.chunk, 0);
    assert_eq!(increasing_uniform.chunk_remaining, 1);
}

