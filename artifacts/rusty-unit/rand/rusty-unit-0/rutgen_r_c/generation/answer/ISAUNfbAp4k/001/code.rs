// Answer 0

#[test]
fn test_reseed_success() {
    struct MockRng;
    
    impl BlockRngCore for MockRng {}
    
    impl SeedableRng for MockRng {
        type Seed = [u8; 32];
        
        fn from_seed(_: Self::Seed) -> Self {
            MockRng
        }
    }
    
    struct MockReseeder;
    
    impl TryRngCore for MockReseeder {
        type Error = rand_core::OsError;
        
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    
    let threshold: u64 = 1024;
    let reseeder = MockReseeder;
    
    let mut rng = ReseedingRng::new(threshold, reseeder).unwrap();
    let mut thread_rng = ThreadRng { rng: Rc::new(UnsafeCell::new(rng)) };
    
    let result = thread_rng.reseed();
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_reseed_panic() {
    struct PanicRng;
    
    impl BlockRngCore for PanicRng {}
    
    impl SeedableRng for PanicRng {
        type Seed = [u8; 32];
        
        fn from_seed(_: Self::Seed) -> Self {
            PanicRng
        }
    }
    
    struct PanicReseeder;
    
    impl TryRngCore for PanicReseeder {
        type Error = rand_core::OsError;
        
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
            Err(rand_core::OsError::new())
        }
    }
    
    let threshold: u64 = 1024;
    let reseeder = PanicReseeder;
    
    let mut rng = ReseedingRng::new(threshold, reseeder).unwrap();
    let mut thread_rng = ThreadRng { rng: Rc::new(UnsafeCell::new(rng)) };
    
    thread_rng.reseed().unwrap(); // This will panic in the `try_fill_bytes` call of PanicReseeder
}

