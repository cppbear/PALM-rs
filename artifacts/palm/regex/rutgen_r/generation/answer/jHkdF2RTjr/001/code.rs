// Answer 0

#[test]
fn test_empty_literals_with_limits() {
    struct TestStruct {
        flag_literal_limit: usize,
        flag_class_limit: usize,
    }
    
    impl TestStruct {
        fn empty_literals(&self) -> Literals {
            let mut lits = Literals::empty();
            lits.set_limit_size(self.flag_literal_limit);
            lits.set_limit_class(self.flag_class_limit);
            lits
        }
    }
    
    struct Literals {
        limit_size: usize,
        limit_class: usize,
    }
    
    impl Literals {
        fn empty() -> Self {
            Literals {
                limit_size: 0,
                limit_class: 0,
            }
        }
        
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }
        
        fn set_limit_class(&mut self, class: usize) {
            self.limit_class = class;
        }
    }
    
    let test_instance = TestStruct { flag_literal_limit: 10, flag_class_limit: 5 };
    let expected = Literals { limit_size: 10, limit_class: 5 };
    
    let result = test_instance.empty_literals();
    
    assert_eq!(result.limit_size, expected.limit_size);
    assert_eq!(result.limit_class, expected.limit_class);
}

#[test]
fn test_empty_literals_with_zero_limits() {
    struct TestStruct {
        flag_literal_limit: usize,
        flag_class_limit: usize,
    }
    
    impl TestStruct {
        fn empty_literals(&self) -> Literals {
            let mut lits = Literals::empty();
            lits.set_limit_size(self.flag_literal_limit);
            lits.set_limit_class(self.flag_class_limit);
            lits
        }
    }
    
    struct Literals {
        limit_size: usize,
        limit_class: usize,
    }
    
    impl Literals {
        fn empty() -> Self {
            Literals {
                limit_size: 0,
                limit_class: 0,
            }
        }
        
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }
        
        fn set_limit_class(&mut self, class: usize) {
            self.limit_class = class;
        }
    }
    
    let test_instance = TestStruct { flag_literal_limit: 0, flag_class_limit: 0 };
    let expected = Literals { limit_size: 0, limit_class: 0 };
    
    let result = test_instance.empty_literals();
    
    assert_eq!(result.limit_size, expected.limit_size);
    assert_eq!(result.limit_class, expected.limit_class);
}

