// Answer 0

#[test]
fn test_inst_ptrs() {
    struct TestStruct {
        data: Vec<u8>,
    }

    impl TestStruct {
        fn inst_ptrs(&self) -> InstPtrs {
            InstPtrs {
                base: 0,
                data: &self.data[1..],
            }
        }
    }

    struct InstPtrs<'a> {
        base: usize,
        data: &'a [u8],
    }

    let test_instance = TestStruct {
        data: vec![0, 1, 2, 3, 4],
    };
    let inst_ptrs = test_instance.inst_ptrs();
    
    assert_eq!(inst_ptrs.base, 0);
    assert_eq!(inst_ptrs.data, &[1, 2, 3, 4]);
}

#[test]
fn test_inst_ptrs_empty_data() {
    struct TestStruct {
        data: Vec<u8>,
    }

    impl TestStruct {
        fn inst_ptrs(&self) -> InstPtrs {
            InstPtrs {
                base: 0,
                data: &self.data[1..],
            }
        }
    }

    struct InstPtrs<'a> {
        base: usize,
        data: &'a [u8],
    }

    let test_instance = TestStruct {
        data: vec![0],
    };
    let inst_ptrs = test_instance.inst_ptrs();
    
    assert_eq!(inst_ptrs.base, 0);
    assert_eq!(inst_ptrs.data, &[]);
}

