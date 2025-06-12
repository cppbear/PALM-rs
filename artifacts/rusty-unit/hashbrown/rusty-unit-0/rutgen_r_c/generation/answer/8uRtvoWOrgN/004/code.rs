// Answer 0

#[test]
fn test_get_many_mut_unique_hashes() {
    use std::ptr::NonNull;
    use std::alloc::{Layout, Global};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simple allocation for testing
            let layout = layout;
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    struct TestTable {
        data: Vec<u64>,
    }

    impl TestTable {
        fn new() -> Self {
            TestTable { data: Vec::new() }
        }

        fn insert(&mut self, hash: u64) {
            self.data.push(hash);
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &u64) -> bool,
        ) -> [Option<&'_ mut u64>; N] {
            let mut results: [Option<&mut u64>; N] = Default::default();
            for (i, hash) in hashes.iter().enumerate() {
                if let Some(pos) = self.data.iter_mut().position(|x| eq(i, x)) {
                    results[i] = Some(&mut self.data[pos]);
                } else {
                    results[i] = None;
                }
            }
            results
        }
    }

    let mut table = TestTable::new();
    table.insert(1);
    table.insert(2);
    table.insert(3);

    let hashes = [1, 2, 3];
    let mut eq = |i, k| k == &hashes[i];

    let result = table.get_many_mut(hashes, eq);
    assert_eq!(result[0], Some(&mut 1));
    assert_eq!(result[1], Some(&mut 2));
    assert_eq!(result[2], Some(&mut 3));
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_duplicate_hashes() {
    use std::ptr::NonNull;
    use std::alloc::{Layout, Global};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let layout = layout;
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    struct TestTable {
        data: Vec<u64>,
    }

    impl TestTable {
        fn new() -> Self {
            TestTable { data: Vec::new() }
        }

        fn insert(&mut self, hash: u64) {
            self.data.push(hash);
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &u64) -> bool,
        ) -> [Option<&'_ mut u64>; N] {
            let mut results: [Option<&mut u64>; N] = Default::default();
            let mut ptrs = [None; N];
            for (i, hash) in hashes.iter().enumerate() {
                if let Some(pos) = self.data.iter_mut().position(|x| eq(i, x)) {
                    ptrs[i] = Some(&mut self.data[pos]);
                } else {
                    ptrs[i] = None;
                }
            }

            for (i, cur) in ptrs.iter().enumerate() {
                if cur.is_some() && ptrs[..i].contains(cur) {
                    panic!("duplicate keys found");
                }
            }

            let mut final_results = Default::default();
            for (i, ptr) in ptrs.iter().enumerate() {
                final_results[i] = ptr.map(|&mut ptr| ptr);
            }
            final_results
        }
    }

    let mut table = TestTable::new();
    table.insert(1);
    table.insert(2);
    table.insert(3);

    let hashes = [1, 1, 2]; // contains duplicate hash
    let mut eq = |i, k| k == &hashes[i];

    let _ = table.get_many_mut(hashes, eq); // This should panic
}

