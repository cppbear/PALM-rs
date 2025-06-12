// Answer 0

#[test]
fn test_sample_single_valid_range() {
    use rand::rngs::ThreadRng;
    use rand::RngCore;
    use rand::distributions::{Uniform, Distribution};

    struct TestRng(ThreadRng);

    impl RngCore for TestRng {
        // Implement necessary methods for RngCore
        fn next_u32(&mut self) -> u32 {
            rand::random::<u32>()
        }

        fn next_u64(&mut self) -> u64 {
            rand::random::<u64>()
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.0.fill_bytes(dest)
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let rng = &mut TestRng(rand::thread_rng());
    let distribution = Uniform::new_inclusive(1, 10);
    
    let result = distribution.sample_single(rng);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value >= 1 && value <= 10);
}

#[test]
#[should_panic]
fn test_sample_single_panic_empty_range() {
    use rand::rngs::ThreadRng;
    use rand::RngCore;
    use rand::distributions::{Uniform};

    struct TestRng(ThreadRng);

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            rand::random::<u32>()
        }

        fn next_u64(&mut self) -> u64 {
            rand::random::<u64>()
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.0.fill_bytes(dest)
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let rng = &mut TestRng(rand::thread_rng());
    let distribution = Uniform::new_inclusive(5, 4); // Empty range, should panic
    let _ = distribution.sample_single(rng);
}

