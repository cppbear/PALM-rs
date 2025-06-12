// Answer 0

#[test]
fn test_fmt_debug_list() {
    use std::fmt;
    use indexmap::IndexMap;

    struct DebuggableStruct {
        data: IndexMap<i32, i32>,
    }

    impl fmt::Debug for DebuggableStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.clone()).finish()
        }
    }

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let debuggable = DebuggableStruct { data: map };

    let expected_result = "[1: 10, 2: 20]"; // Adjust this to match the expected format

    let actual_result = format!("{:?}", debuggable);

    assert_eq!(actual_result, expected_result);
}

