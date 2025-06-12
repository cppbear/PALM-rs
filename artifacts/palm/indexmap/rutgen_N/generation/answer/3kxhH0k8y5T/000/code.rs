// Answer 0

#[test]
fn test_borrow_mut_success() {
    use indexmap::IndexMap;
    use std::cell::RefMut;

    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                indices: vec![],
                entries: vec![],
            }
        }

        fn borrow_mut(&mut self) -> RefMut<'_, Vec<usize>, Vec<i32>> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    map.indices.push(1);
    map.entries.push(10);

    {
        let mut ref_mut = map.borrow_mut();
        ref_mut.0.push(2);
        ref_mut.1.push(20);
    }

    assert_eq!(map.indices.len(), 2);
    assert_eq!(map.entries.len(), 2);
}

#[test]
#[should_panic]
fn test_borrow_mut_failure() {
    use indexmap::IndexMap;
    use std::cell::RefMut;

    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                indices: vec![],
                entries: vec![],
            }
        }

        fn borrow_mut(&mut self) -> RefMut<'_, Vec<usize>, Vec<i32>> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }

    let mut map = TestMap::new();
    map.indices.push(1);
    map.entries.push(10);
    
    let _ref1 = map.borrow_mut();

    // Trying to borrow_mut again while the first mutable borrow is still active
    let _ref2 = map.borrow_mut(); 
}

