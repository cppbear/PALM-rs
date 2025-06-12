// Answer 0

#[test]
fn test_get_many_unchecked_mut() {
    struct TestStruct {
        value: u64,
    }

    impl TestStruct {
        fn new(value: u64) -> Self {
            TestStruct { value }
        }
    }

    struct TestContainer {
        items: Vec<TestStruct>,
    }

    impl TestContainer {
        fn new() -> Self {
            TestContainer { items: Vec::new() }
        }

        unsafe fn get_many_mut_pointers<const N: usize>(
            &mut self, hashes: [u64; N], eq: impl FnMut(usize, &TestStruct) -> bool
        ) -> [Option<*mut TestStruct>; N] {
            let mut result = [None; N];
            for (i, &hash) in hashes.iter().enumerate() {
                for (j, item) in self.items.iter_mut().enumerate() {
                    if eq(j, item) {
                        result[i] = Some(item as *mut TestStruct);
                        break;
                    }
                }
            }
            result
        }
    }

    let mut container = TestContainer::new();
    container.items.push(TestStruct::new(1));
    container.items.push(TestStruct::new(2));
    container.items.push(TestStruct::new(3));

    let hashes = [0, 1, 2];
    let eq = |index: usize, _: &TestStruct| index == hashes[index] as usize;

    let result: [Option<&mut TestStruct>; 3] = unsafe {
        let mut ptrs = container.get_many_mut_pointers(hashes, eq);
        ptrs.map(|ptr| ptr.map(|mut ptr| ptr.as_mut()))
    };

    assert_eq!(result[0].as_ref().map(|v| v.value), Some(&1));
    assert_eq!(result[1].as_ref().map(|v| v.value), Some(&2));
    assert_eq!(result[2].as_ref().map(|v| v.value), Some(&3));
}

