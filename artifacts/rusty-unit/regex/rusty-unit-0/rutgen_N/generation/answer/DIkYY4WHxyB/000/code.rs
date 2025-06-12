// Answer 0

#[test]
fn test_len() {
    struct TestStruct;

    impl TestStruct {
        pub fn len(&self) -> usize { 
            0 
        }
    }

    let test_instance = TestStruct;
    assert_eq!(test_instance.len(), 0);
}

