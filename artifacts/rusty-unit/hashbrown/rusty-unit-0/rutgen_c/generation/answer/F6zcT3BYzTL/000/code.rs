// Answer 0

#[test]
fn test_difference_debug_fmt() {
    use std::hash::{Hash, Hasher};
    use std::collections::HashSet;
    use std::fmt;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct MockHasher {
        value: u64,
    }

    impl Default for MockHasher {
        fn default() -> Self {
            Self { value: 0 }
        }
    }

    impl Hasher for MockHasher {
        fn finish(&self) -> u64 {
            self.value
        }

        fn write(&mut self, _bytes: &[u8]) {
            self.value = self.value.wrapping_add(1);
        }
    }

    struct AlwaysEqual;

    impl PartialEq for AlwaysEqual {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl Eq for AlwaysEqual {}

    impl Hash for AlwaysEqual {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    let hasher = DefaultHashBuilder::default();
    let mut set = HashSet::with_hasher(hasher);
    set.insert(AlwaysEqual);

    let diff = Difference {
        iter: Iter { inner: RawIter::default(), marker: PhantomData },
        other: &set,
    };

    let result = format!("{:?}", diff);
    assert!(result.contains("AlwaysEqual"));
}

