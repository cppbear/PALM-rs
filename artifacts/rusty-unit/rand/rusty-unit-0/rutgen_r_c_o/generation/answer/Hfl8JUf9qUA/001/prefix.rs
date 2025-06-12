// Answer 0

#[test]
fn test_try_fill_bytes_min_size() {
    struct TestRng;
    
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            42
        }
        
        fn next_u64(&mut self) -> u64 {
            42
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 42;
            }
        }
    }
    
    let mut rng = TestRng;
    let mut bytes: [u8; 1] = [0];
    let _ = rng.try_fill_bytes(&mut bytes);
}

#[test]
fn test_try_fill_bytes_small_size() {
    struct TestRng;
    
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            42
        }
        
        fn next_u64(&mut self) -> u64 {
            42
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 42;
            }
        }
    }
    
    let mut rng = TestRng;
    let mut bytes: [u8; 10] = [0; 10];
    let _ = rng.try_fill_bytes(&mut bytes);
}

#[test]
fn test_try_fill_bytes_large_size() {
    struct TestRng;
    
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            42
        }
        
        fn next_u64(&mut self) -> u64 {
            42
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 42;
            }
        }
    }
    
    let mut rng = TestRng;
    let mut bytes: [u8; 256] = [0; 256];
    let _ = rng.try_fill_bytes(&mut bytes);
}

#[test]
fn test_try_fill_bytes_max_size() {
    struct TestRng;
    
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            42
        }
        
        fn next_u64(&mut self) -> u64 {
            42
        }
        
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 42;
            }
        }
    }
    
    let mut rng = TestRng;
    let mut bytes: [u8; 1024] = [0; 1024];
    let _ = rng.try_fill_bytes(&mut bytes);
}

