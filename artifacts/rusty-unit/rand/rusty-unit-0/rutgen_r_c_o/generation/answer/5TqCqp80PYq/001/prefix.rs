// Answer 0

#[test]
fn test_fill_bytes_empty_array() {
    struct RngMock;
    
    impl TryRngCore for RngMock {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> { Ok(()) }
    }
    
    let mut rng = RngMock;
    let mut buffer: [u8; 0] = [];
    let mut unwrap_mut = UnwrapMut(&mut rng);
    unwrap_mut.fill_bytes(&mut buffer);
}

#[test]
fn test_fill_bytes_small_array() {
    struct RngMock;
    
    impl TryRngCore for RngMock {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> { 
            dst.copy_from_slice(&[1, 2, 3]); 
            Ok(()) 
        }
    }
    
    let mut rng = RngMock;
    let mut buffer: [u8; 3] = [0; 3];
    let mut unwrap_mut = UnwrapMut(&mut rng);
    unwrap_mut.fill_bytes(&mut buffer);
}

#[test]
fn test_fill_bytes_medium_array() {
    struct RngMock;
    
    impl TryRngCore for RngMock {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            for i in 0..dst.len() {
                dst[i] = i as u8;
            }
            Ok(())
        }
    }
    
    let mut rng = RngMock;
    let mut buffer: [u8; 10] = [0; 10];
    let mut unwrap_mut = UnwrapMut(&mut rng);
    unwrap_mut.fill_bytes(&mut buffer);
}

#[test]
fn test_fill_bytes_large_array() {
    struct RngMock;
    
    impl TryRngCore for RngMock {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            for i in 0..dst.len() {
                dst[i] = (i % 256) as u8;
            }
            Ok(())
        }
    }
    
    let mut rng = RngMock;
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut unwrap_mut = UnwrapMut(&mut rng);
    unwrap_mut.fill_bytes(&mut buffer);
}

