// Answer 0

#[test]
fn test_patterns_empty() {
    struct TestStruct;

    impl TestStruct {
        pub fn patterns(&self) -> &[Vec<u8>] { &[] }
    }

    let test_struct = TestStruct;
    let result = test_struct.patterns();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_patterns_non_empty() {
    struct TestStruct {
        data: Vec<Vec<u8>>,
    }

    impl TestStruct {
        pub fn new(data: Vec<Vec<u8>>) -> Self {
            TestStruct { data }
        }

        pub fn patterns(&self) -> &[Vec<u8>] { &self.data }
    }

    let test_struct = TestStruct::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let result = test_struct.patterns();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], vec![1, 2, 3]);
    assert_eq!(result[1], vec![4, 5, 6]);
}

