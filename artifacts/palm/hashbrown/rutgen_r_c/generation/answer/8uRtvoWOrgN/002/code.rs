// Answer 0

#[test]
fn test_get_many_mut_no_duplicates_found() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table: RawTable<i32, DummyAllocator> = RawTable::new_in(DummyAllocator);
    // Assuming some insertions happened here, we will simulate with dummy values:
    let hashes = [1, 2, 3];
    
    // Manually create a condition where the hashes exist
    unsafe {
        // Directly insert values to simulate, as this is just a test setup
        table.insert(1, 10, |x| x.clone() as u64);
        table.insert(2, 20, |x| x.clone() as u64);
        table.insert(3, 30, |x| x.clone() as u64);
    }
    
    let output = table.get_many_mut(hashes, |i, k| {
        match i {
            0 => *k == 10,
            1 => *k == 20,
            2 => *k == 30,
            _ => false,
        }
    });
    
    assert_eq!(output, [Some(&mut 10), Some(&mut 20), Some(&mut 30)]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_duplicates_trigger_panic() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table: RawTable<i32, DummyAllocator> = RawTable::new_in(DummyAllocator);
    // Assuming some insertions happened here, we will simulate with dummy values:
    let hashes = [1, 1, 2]; // Duplicate hash

    unsafe {
        // Directly insert values to simulate, as this is just a test setup
        table.insert(1, 10, |x| x.clone() as u64);
        table.insert(2, 20, |x| x.clone() as u64);
    }

    let _output = table.get_many_mut(hashes, |i, k| {
        match i {
            0 => *k == 10,
            1 => *k == 10, // This will cause a duplicate case
            2 => *k == 20,
            _ => false,
        }
    });
}

#[test]
fn test_get_many_mut_not_found() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table: RawTable<i32, DummyAllocator> = RawTable::new_in(DummyAllocator);
    // Assuming some insertions happened here, we will simulate with dummy values:
    let hashes = [1, 2, 3]; // None of these exist

    unsafe {
        // Directly insert values to simulate, as this is just a test setup
        table.insert(4, 40, |x| x.clone() as u64);
        table.insert(5, 50, |x| x.clone() as u64);
    }

    let output = table.get_many_mut(hashes, |i, k| {
        match i {
            0 => false,
            1 => false,
            2 => false,
            _ => false,
        }
    });

    assert_eq!(output, [None, None, None]);
}

