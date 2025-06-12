// Answer 0

#[test]
fn test_clone_from_impl_success() {
    struct TestStruct {
        value: i32,
    }

    struct Table {
        ctrl_bytes: usize,
        items: usize,
        growth_left: usize,
    }

    struct Test {
        table: Table,
        buckets: Vec<Option<TestStruct>>,
    }

    impl Test {
        fn new(ctrl_bytes: usize, items: usize, growth_left: usize) -> Self {
            Self {
                table: Table {
                    ctrl_bytes,
                    items,
                    growth_left,
                },
                buckets: vec![None; items],
            }
        }

        fn iter(&self) -> impl Iterator<Item = &TestStruct> {
            self.buckets.iter().filter_map(|b| b.as_ref())
        }

        fn bucket_index(&self, item: &TestStruct) -> usize {
            self.buckets.iter().position(|b| b.as_ref() == Some(item)).unwrap()
        }

        fn is_bucket_full(&self, index: usize) -> bool {
            self.buckets[index].is_some()
        }

        fn bucket(&mut self, index: usize) -> &mut Option<TestStruct> {
            &mut self.buckets[index]
        }
    }

    let mut source = Test::new(10, 5, 3);
    source.bucket(0).replace(TestStruct { value: 1 });
    source.bucket(1).replace(TestStruct { value: 2 });

    let mut target = Test::new(10, 5, 3);
    
    unsafe {
        target.clone_from_impl(&source);
    }
    
    assert!(target.buckets[0].is_some());
    assert_eq!(target.buckets[0].as_ref().unwrap().value, 1);
    assert!(target.buckets[1].is_some());
    assert_eq!(target.buckets[1].as_ref().unwrap().value, 2);
}

#[test]
#[should_panic]
fn test_clone_from_impl_panics_on_empty_source() {
    struct TestStruct {
        value: i32,
    }

    struct Table {
        ctrl_bytes: usize,
        items: usize,
        growth_left: usize,
    }

    struct Test {
        table: Table,
        buckets: Vec<Option<TestStruct>>,
    }
    
    impl Test {
        fn new(ctrl_bytes: usize, items: usize, growth_left: usize) -> Self {
            Self {
                table: Table {
                    ctrl_bytes,
                    items,
                    growth_left,
                },
                buckets: vec![None; items],
            }
        }

        fn iter(&self) -> impl Iterator<Item = &TestStruct> {
            self.buckets.iter().filter_map(|b| b.as_ref())
        }

        fn bucket_index(&self, _item: &TestStruct) -> usize {
            panic!("This should not be called when source is empty");
        }

        fn is_bucket_full(&self, _: usize) -> bool {
            false
        }

        fn bucket(&mut self, _: usize) -> &mut Option<TestStruct> {
            panic!("This should not be called when source is empty");
        }
    }

    let source = Test::new(10, 0, 3); // No items
    let mut target = Test::new(10, 5, 3);
    
    unsafe {
        target.clone_from_impl(&source);
    }
}

