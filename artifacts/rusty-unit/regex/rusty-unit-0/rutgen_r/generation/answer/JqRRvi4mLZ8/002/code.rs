// Answer 0

#[test]
fn test_check_size_equal_limit() {
    use std::mem::size_of;
    
    struct Inst;
    
    struct Checker {
        insts: Vec<Inst>,
        size_limit: usize,
    }
    
    impl Checker {
        fn check_size(&self) -> Result<(), &'static str> {
            use std::mem::size_of;

            if self.insts.len() * size_of::<Inst>() > self.size_limit {
                Err("CompiledTooBig")
            } else {
                Ok(())
            }
        }
    }
    
    let insts = vec![Inst; 2]; // Assuming size_of::<Inst>() is 8 bytes, total = 2 * 8 = 16 bytes
    let size_limit = 16; // Set the size limit to match the total size
    
    let checker = Checker {
        insts,
        size_limit,
    };
    
    let result = checker.check_size();
    assert!(result.is_ok());
}

#[test]
fn test_check_size_less_than_limit() {
    use std::mem::size_of;
    
    struct Inst;
    
    struct Checker {
        insts: Vec<Inst>,
        size_limit: usize,
    }
    
    impl Checker {
        fn check_size(&self) -> Result<(), &'static str> {
            use std::mem::size_of;

            if self.insts.len() * size_of::<Inst>() > self.size_limit {
                Err("CompiledTooBig")
            } else {
                Ok(())
            }
        }
    }
    
    let insts = vec![Inst; 1]; // Assuming size_of::<Inst>() is 8 bytes, total = 1 * 8 = 8 bytes
    let size_limit = 16; // Set the size limit greater than the total size
    
    let checker = Checker {
        insts,
        size_limit,
    };
    
    let result = checker.check_size();
    assert!(result.is_ok());
}

