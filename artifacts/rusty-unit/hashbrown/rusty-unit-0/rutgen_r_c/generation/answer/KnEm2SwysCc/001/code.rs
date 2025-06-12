// Answer 0

#[test]
fn test_into_iter_debug_fmt() {
    use std::fmt;
    use std::alloc::Layout;
    use std::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::from(&mut 0u8))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = DummyAllocator;

    let key_value_pairs = vec![(1, "one"), (2, "two"), (3, "three")];

    struct RawIterStub<'a> {
        data: &'a [(i32, &str)],
        index: usize,
    }

    impl<'a> RawIterStub<'a> {
        fn new(data: &'a [(i32, &str)]) -> Self {
            RawIterStub { data, index: 0 }
        }

        fn next(&mut self) -> Option<&'a (i32, &str)> {
            if self.index < self.data.len() {
                let item = &self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct RawIntoIterStub<K, V> {
        iter: RawIterStub<(K, V)>,
    }

    impl<K, V> RawIntoIterStub<K, V> {
        fn iter(&self) -> &RawIterStub<(K, V)> {
            &self.iter
        }
    }

    let raw_into_iter = RawIntoIterStub {
        iter: RawIterStub::new(&key_value_pairs),
    };

    let into_iter = IntoIter {
        inner: raw_into_iter,
    };

    let mut output = String::new();
    let result = fmt::write(&mut output, |f| into_iter.fmt(f));

    assert!(result.is_ok());
    assert_eq!(output, "[(1, \"one\"), (2, \"two\"), (3, \"three\")]");
}

