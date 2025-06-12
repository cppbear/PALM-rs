// Answer 0

#[test]
fn test_into_values_debug_fmt() {
    use core::fmt;
    use core::marker::PhantomData;
    
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    struct TestIntoIter<K, V> {
        items: Vec<(K, V)>,
        index: usize,
    }

    impl<K, V> TestIntoIter<K, V> {
        fn new(items: Vec<(K, V)>) -> Self {
            Self { items, index: 0 }
        }

        fn iter(&self) -> impl Iterator<Item = &(K, V)> {
            self.items.iter()
        }
    }

    impl<K: Debug, V: Debug> fmt::Debug for TestIntoValues<K, V, TestAllocator> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.inner.iter().map(|(_, v)| v)).finish()
        }
    }

    struct TestIntoValues<K, V, A: Allocator> {
        inner: TestIntoIter<K, V>,
    }

    let test_values = vec![(1, "a"), (2, "b"), (3, "c")];
    let test_into_values = TestIntoValues {
        inner: TestIntoIter::new(test_values),
    };

    let mut buffer = Vec::new();
    let writer = &mut fmt::Write::write_str(writer, ""); 
    write!(writer, "{:?}", test_into_values).unwrap();

    let expected_output = "[\"a\", \"b\", \"c\"]"; 
    assert_eq!(writer.contents(), expected_output);
}

