// Answer 0

#[test]
fn test_next_no_items() {
    struct TestStruct {
        iter: Vec<Option<i32>>,
        table: std::collections::HashMap<*const i32, i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            Self {
                iter: Vec::new(),
                table: std::collections::HashMap::new(),
            }
        }

        pub(crate) fn next<F>(&mut self, mut f: F) -> Option<i32>
        where
            F: FnMut(&mut i32) -> bool,
        {
            unsafe {
                for item in &mut self.iter {
                    if let Some(ref mut val) = item {
                        if f(val) {
                            return self.table.remove(item.as_mut().unwrap()).map(|x| x);
                        }
                    }
                }
            }
            None
        }
    }

    let mut test_struct = TestStruct::new();    
    let result = test_struct.next(|_| true);
    assert!(result.is_none());
}

#[test]
fn test_next_with_item_false() {
    struct TestStruct {
        iter: Vec<Option<i32>>,
        table: std::collections::HashMap<*const i32, i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            let mut table = std::collections::HashMap::new();
            let item = 42;

            table.insert(&item as *const _ as *const i32, item);
            Self {
                iter: vec![Some(item)],
                table,
            }
        }

        pub(crate) fn next<F>(&mut self, mut f: F) -> Option<i32>
        where
            F: FnMut(&mut i32) -> bool,
        {
            unsafe {
                for item in &mut self.iter {
                    if let Some(ref mut val) = item {
                        if f(val) {
                            return self.table.remove(item.as_mut().unwrap()).map(|x| x);
                        }
                    }
                }
            }
            None
        }
    }

    let mut test_struct = TestStruct::new();    
    let result = test_struct.next(|_| false);
    assert!(result.is_none());
}

