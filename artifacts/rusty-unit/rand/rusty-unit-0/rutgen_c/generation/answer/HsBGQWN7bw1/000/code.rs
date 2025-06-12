// Answer 0

#[test]
fn test_generate_reseed_needed() {
    use rand_core::block::{BlockRng, CryptoBlockRng};
    use rand_core::{SeedableRng, RngCore, Rng};

    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u8;
        type Results = [u8; 16];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&[1; 16]);
        }
    }

    struct MockSeeder;

    impl TryRngCore for MockSeeder {
        type Error = ();
        
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(MockSeeder)
        }
    }

    let mut reseeding_rng: ReseedingCore<MockRng, MockSeeder> = 
        ReseedingCore::new(16, MockSeeder).unwrap();

    let mut results = [0u8; 16];
    reseeding_rng.bytes_until_reseed = 0;

    reseeding_rng.generate(&mut results);

    assert_eq!(results, [1; 16]);
}

#[test]
fn test_generate_no_reseed_needed() {
    use rand_core::block::{BlockRng, CryptoBlockRng};
    use rand_core::{SeedableRng, RngCore, Rng};

    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u8;
        type Results = [u8; 16];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&[2; 16]);
        }
    }

    struct MockSeeder;

    impl TryRngCore for MockSeeder {
        type Error = ();
        
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(MockSeeder)
        }
    }

    let mut reseeding_rng: ReseedingCore<MockRng, MockSeeder> = 
        ReseedingCore::new(16, MockSeeder).unwrap();

    let mut results = [0u8; 16];
    reseeding_rng.bytes_until_reseed = 16;

    reseeding_rng.generate(&mut results);

    assert_eq!(results, [2; 16]);
} 

#[test]
fn test_generate_bytes_until_reseed_depletes() {
    use rand_core::block::{BlockRng, CryptoBlockRng};
    use rand_core::{SeedableRng, RngCore, Rng};

    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u8;
        type Results = [u8; 16];

        fn generate(&mut self, results: &mut Self::Results) {
            results.copy_from_slice(&[3; 16]);
        }
    }

    struct MockSeeder;

    impl TryRngCore for MockSeeder {
        type Error = ();
        
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, Self::Error> {
            Ok(MockSeeder)
        }
    }

    let mut reseeding_rng: ReseedingCore<MockRng, MockSeeder> = 
        ReseedingCore::new(16, MockSeeder).unwrap();

    let mut results = [0u8; 16];
    reseeding_rng.bytes_until_reseed = 1;

    reseeding_rng.generate(&mut results);
    
    assert_eq!(results, [3; 16]);
    assert!(reseeding_rng.bytes_until_reseed < 0); // Ensure the reseeding occurred.
}

