// Answer 0

#[test]
fn test_fmt_debug_list_non_empty() {
    use std::fmt;
    use hashbrown::HashMap;

    struct MyStruct {
        map: HashMap<i32, i32>,
    }

    impl MyStruct {
        fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(1, 100);
            map.insert(2, 200);
            MyStruct { map }
        }
    }

    impl fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.map.iter()).finish()
        }
    }

    let my_struct = MyStruct::new();
    let result = format!("{:?}", my_struct);
    assert_eq!(result, "[(1, 100), (2, 200)]");
}

#[test]
fn test_fmt_debug_list_empty() {
    use std::fmt;
    use hashbrown::HashMap;

    struct MyStruct {
        map: HashMap<i32, i32>,
    }

    impl MyStruct {
        fn new() -> Self {
            MyStruct { map: HashMap::new() }
        }
    }

    impl fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.map.iter()).finish()
        }
    }

    let my_struct = MyStruct::new();
    let result = format!("{:?}", my_struct);
    assert_eq!(result, "[]");
}

