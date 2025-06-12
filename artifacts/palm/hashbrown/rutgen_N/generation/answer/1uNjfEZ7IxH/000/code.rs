// Answer 0

#[test]
fn test_get_many_mut_pointers_valid_case() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    struct TestHashMap {
        data: Vec<TestStruct>,
    }

    impl TestHashMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn insert(&mut self, value: i32) {
            self.data.push(TestStruct { value });
        }

        unsafe fn find(&mut self, hash: u64, eq: impl FnMut(usize, &TestStruct) -> bool) -> Option<&mut TestStruct> {
            let index = (hash % self.data.len() as u64) as usize;
            if eq(index, &self.data[index]) {
                Some(&mut self.data[index])
            } else {
                None
            }
        }

        unsafe fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &TestStruct) -> bool,
        ) -> [Option<NonNull<TestStruct>>; N] {
            array::from_fn(|i| {
                self.find(hashes[i], |k| eq(i, k))
                    .map(|cur| NonNull::from(cur))
            })
        }
    }

    let mut map = TestHashMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);

    let hashes: [u64; 3] = [0, 1, 2];
    let pointers = unsafe { map.get_many_mut_pointers(hashes, |i, k| k.value == (i as i32 + 1)) };

    assert!(pointers[0].is_some());
    assert!(pointers[1].is_some());
    assert!(pointers[2].is_some());
}

#[test]
fn test_get_many_mut_pointers_empty_case() {
    use std::ptr::NonNull;

    struct TestStruct {
        value: i32,
    }

    struct TestHashMap {
        data: Vec<TestStruct>,
    }

    impl TestHashMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        unsafe fn find(&mut self, _hash: u64, _eq: impl FnMut(usize, &TestStruct) -> bool) -> Option<&mut TestStruct> {
            None
        }

        unsafe fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &TestStruct) -> bool,
        ) -> [Option<NonNull<TestStruct>>; N] {
            array::from_fn(|i| {
                self.find(hashes[i], |k| eq(i, k))
                    .map(|cur| NonNull::from(cur))
            })
        }
    }

    let mut map = TestHashMap::new();

    let hashes: [u64; 3] = [0, 1, 2];
    let pointers = unsafe { map.get_many_mut_pointers(hashes, |_, _| false) };

    assert!(pointers.iter().all(|p| p.is_none()));
}

