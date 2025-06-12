// Answer 0

#[test]
fn test_next_u32_initial_state() {
    struct TestCore {
        values: Vec<u32>,
        index: usize,
    }

    impl TestCore {
        fn generate(&mut self, results: &mut [u32]) {
            results.copy_from_slice(&self.values);
        }
    }
    
    struct RandomNumbers {
        core: TestCore,
        results: Vec<u32>,
        index: usize,
        half_used: bool,
    }

    let core = TestCore {
        values: vec![0xFFFF_FFFF, 0xAAAA_AAAA, 0x5555_5555],
        index: 0,
    };
    
    let mut random_numbers = RandomNumbers {
        core,
        results: vec![0; 3],
        index: 0,
        half_used: false,
    };

    let first = random_numbers.next_u32();
    assert_eq!(first, 0xFFFF_FFFF);
    
    let second = random_numbers.next_u32();
    assert_eq!(second, 0xAAAA_AAAA);
    
    let third = random_numbers.next_u32();
    assert_eq!(third, 0x5555_5555);
}

#[test]
fn test_next_u32_full_cycle() {
    struct TestCore {
        values: Vec<u32>,
        index: usize,
    }

    impl TestCore {
        fn generate(&mut self, results: &mut [u32]) {
            results.copy_from_slice(&self.values);
        }
    }
    
    struct RandomNumbers {
        core: TestCore,
        results: Vec<u32>,
        index: usize,
        half_used: bool,
    }

    let core = TestCore {
        values: vec![0x1234_5678, 0x9ABC_DEF0],
        index: 0,
    };
    
    let mut random_numbers = RandomNumbers {
        core,
        results: vec![0; 2],
        index: 0,
        half_used: false,
    };

    let first = random_numbers.next_u32();
    assert_eq!(first, 0x1234_5678);

    let second = random_numbers.next_u32();
    assert_eq!(second, 0x9ABC_DEF0);

    let third = random_numbers.next_u32(); // should trigger generation
    assert_eq!(third, 0x1234_5678);
}

#[test]
fn test_next_u32_boundary_condition() {
    struct TestCore {
        values: Vec<u32>,
        index: usize,
    }

    impl TestCore {
        fn generate(&mut self, results: &mut [u32]) {
            results.copy_from_slice(&self.values);
        }
    }
    
    struct RandomNumbers {
        core: TestCore,
        results: Vec<u32>,
        index: usize,
        half_used: bool,
    }

    let core = TestCore {
        values: vec![0xABCD_EF00],
        index: 0,
    };
    
    let mut random_numbers = RandomNumbers {
        core,
        results: vec![0; 1],
        index: 0,
        half_used: false,
    };

    let first = random_numbers.next_u32();
    assert_eq!(first, 0xABCD_EF00);

    let second = random_numbers.next_u32(); // should trigger generation
    assert_eq!(second, 0xABCD_EF00); 
}

