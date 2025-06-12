// Answer 0

#[test]
fn test_symmetric_difference_debug_fmt() {
    use std::collections::HashSet;
    use std::hash::{BuildHasherDefault, Hash};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    let set_a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };

    let mut output = String::new();
    let _ = write!(output, "{:?}", symmetric_difference);
    
    assert!(!output.is_empty());
}

