// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use indexmap::IndexMap;

    struct TestStruct {
        iter: IndexMap<i32, i32>,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.iter, f)
        }
    }

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let test_struct = TestStruct { iter: map };

    let output = format!("{:?}", test_struct);
    assert!(output.contains("1: 10"));
    assert!(output.contains("2: 20"));
}

