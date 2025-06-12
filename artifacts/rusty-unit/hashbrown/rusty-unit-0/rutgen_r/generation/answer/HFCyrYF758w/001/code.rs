// Answer 0

#[test]
fn test_is_bucket_full_valid_index() {
    struct TestTable {
        buckets: Vec<bool>,
    }
    
    impl TestTable {
        fn is_bucket_full(&self, index: usize) -> bool {
            self.buckets[index]
        }
    }
    
    struct TestStruct {
        table: TestTable,
    }

    let test_struct = TestStruct {
        table: TestTable {
            buckets: vec![false, true, false, true, true],
        },
    };

    unsafe {
        assert_eq!(test_struct.is_bucket_full(1), true);
        assert_eq!(test_struct.is_bucket_full(0), false);
    }
}

#[test]
#[should_panic]
fn test_is_bucket_full_out_of_bounds() {
    struct TestTable {
        buckets: Vec<bool>,
    }
    
    impl TestTable {
        fn is_bucket_full(&self, index: usize) -> bool {
            self.buckets[index]
        }
    }
    
    struct TestStruct {
        table: TestTable,
    }

    let test_struct = TestStruct {
        table: TestTable {
            buckets: vec![false, true, false, true, true],
        },
    };

    unsafe {
        test_struct.is_bucket_full(5); // This should trigger a panic due to out of bounds access.
    }
}

#[test]
fn test_is_bucket_full_empty_buckets() {
    struct TestTable {
        buckets: Vec<bool>,
    }
    
    impl TestTable {
        fn is_bucket_full(&self, index: usize) -> bool {
            self.buckets[index]
        }
    }
    
    struct TestStruct {
        table: TestTable,
    }

    let test_struct = TestStruct {
        table: TestTable {
            buckets: vec![false, false, false, false],
        },
    };

    unsafe {
        assert_eq!(test_struct.is_bucket_full(0), false);
        assert_eq!(test_struct.is_bucket_full(3), false);
    }
}

#[test]
fn test_is_bucket_full_all_full() {
    struct TestTable {
        buckets: Vec<bool>,
    }
    
    impl TestTable {
        fn is_bucket_full(&self, index: usize) -> bool {
            self.buckets[index]
        }
    }
    
    struct TestStruct {
        table: TestTable,
    }

    let test_struct = TestStruct {
        table: TestTable {
            buckets: vec![true, true, true, true],
        },
    };

    unsafe {
        assert_eq!(test_struct.is_bucket_full(0), true);
        assert_eq!(test_struct.is_bucket_full(3), true);
    }
}

