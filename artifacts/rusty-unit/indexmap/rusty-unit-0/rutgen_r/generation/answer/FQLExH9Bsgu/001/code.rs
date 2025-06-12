// Answer 0

#[test]
fn test_as_mut_slice() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn as_entries_mut(&mut self) -> &mut [(K, V)] {
            self.entries.as_mut_slice()
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }
    }

    struct Slice<K, V> {
        data: *mut [(K, V)],
        len: usize,
    }

    impl<K, V> Slice<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> &mut Self {
            unsafe {
                &mut *(slice as *mut _ as *mut Slice<K, V>)
            }
        }
    }

    let mut test_map = TestMap::new();
    test_map.push(1, "one");
    test_map.push(2, "two");
    
    let slice = test_map.as_entries_mut();
    let result_slice = Slice::from_mut_slice(slice);
    
    assert_eq!(result_slice.len, test_map.entries.len());
    assert_eq!(unsafe { &*result_slice.data }, &test_map.entries[..]);
}

