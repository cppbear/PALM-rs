// Answer 0

#[test]
fn test_gen_u32_with_max_value() {
    struct TestGen {
        value: u64,
    }
    
    impl TestGen {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut generator = TestGen { value: u64::MAX };
    assert_eq!(generator.gen_u32(), u32::MAX);
}

#[test]
fn test_gen_u32_with_zero() {
    struct TestGen {
        value: u64,
    }
    
    impl TestGen {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut generator = TestGen { value: 0 };
    assert_eq!(generator.gen_u32(), 0);
}

#[test]
fn test_gen_u32_with_mid_value() {
    struct TestGen {
        value: u64,
    }
    
    impl TestGen {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut generator = TestGen { value: 2_u64.pow(32) };
    assert_eq!(generator.gen_u32(), 0);
}

#[test]
fn test_gen_u32_with_boundary_value() {
    struct TestGen {
        value: u64,
    }
    
    impl TestGen {
        fn gen_u64(&mut self) -> u64 {
            self.value
        }
    }

    let mut generator = TestGen { value: 2_u64.pow(32) - 1 };
    assert_eq!(generator.gen_u32(), u32::MAX - 1);
}

