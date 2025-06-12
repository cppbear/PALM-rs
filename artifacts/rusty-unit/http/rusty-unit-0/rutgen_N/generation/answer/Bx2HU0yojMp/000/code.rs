// Answer 0

#[test]
fn test_raw_links() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                entries: vec![1, 2, 3],
            }
        }

        fn raw_links(&mut self) -> *mut RawLinks<i32> {
            RawLinks(&mut self.entries[..] as *mut _)
        }
    }

    struct RawLinks<'a, T>(&'a mut [T]);

    let mut test_struct = TestStruct::new();
    let raw_links_ptr = test_struct.raw_links();
    assert!(!raw_links_ptr.is_null());
}

