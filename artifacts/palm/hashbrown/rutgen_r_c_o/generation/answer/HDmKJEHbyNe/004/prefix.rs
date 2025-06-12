// Answer 0

#[test]
fn test_rehash_in_place_panic_conditions() {
    let size_of = std::mem::size_of::<u64>();
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 16);
    // Assume we have elements to operate on, filling in DELETED states.
    for i in 0..raw_table.buckets() {
        let ptr = raw_table.bucket_ptr(i, size_of);
        unsafe {
            raw_table.set_ctrl(i, Tag::DELETED); // Set all control for a test case.
            ptr::write(ptr, i as u64); // Simulating inserting data.
        }
    }

    let hasher = |table: &mut RawTableInner, index: usize| index as u64;

    // Calling rehash_in_place
    unsafe {
        raw_table.rehash_in_place(&hasher, size_of, Some(|ptr| {
            ptr::drop_in_place(ptr as *mut u64); // Valid drop function.
        }));
    }
}

#[test]
#[should_panic]
fn test_rehash_in_place_panic_existing_ctrl() {
    let size_of = std::mem::size_of::<u64>();
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 16);
    // Prepare table with both DELETED and EMPTY.
    for i in 0..raw_table.buckets() {
        let ptr = raw_table.bucket_ptr(i, size_of);
        unsafe {
            if i % 2 == 0 {
                raw_table.set_ctrl(i, Tag::EMPTY);
            } else {
                raw_table.set_ctrl(i, Tag::DELETED);
                ptr::write(ptr, i as u64);
            }
        }
    }

    let hasher = |table: &mut RawTableInner, index: usize| {
        if index == 0 {
            panic!("This hash function will panic");
        }
        index as u64
    };

    // Will trigger panic due to the hash function.
    unsafe {
        raw_table.rehash_in_place(&hasher, size_of, None);
    }
}

#[test]
fn test_rehash_in_place_in_same_group() {
    let size_of = std::mem::size_of::<u64>();
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 16);
    // Setup the table where items are in the same group.
    for i in 0..raw_table.buckets() {
        if i % 2 == 0 {
            let ptr = raw_table.bucket_ptr(i, size_of);
            unsafe {
                raw_table.set_ctrl(i, Tag::DELETED);
                ptr::write(ptr, i as u64);
            }
        } else {
            unsafe { raw_table.set_ctrl(i, Tag::EMPTY); }
        }
    }

    let hasher = |table: &mut RawTableInner, index: usize| {
        // Ensure that we are returning the same group for certain indices.
        (index % 4) as u64
    };

    unsafe {
        raw_table.rehash_in_place(&hasher, size_of, None);
    }
}

#[test]
fn test_rehash_in_place_not_in_same_group() {
    let size_of = std::mem::size_of::<u64>();
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 16);
    
    // Setup items where they will not share the same group.
    for i in 0..raw_table.buckets() {
        let ptr = raw_table.bucket_ptr(i, size_of);
        unsafe {
            raw_table.set_ctrl(i, Tag::DELETED);
            ptr::write(ptr, (i * 2) as u64); // Inserting differing values.
        }
    }

    let hasher = |table: &mut RawTableInner, index: usize| {
        return (index * 3) as u64; // Different hashes to avoid same group.
    };

    unsafe {
        raw_table.rehash_in_place(&hasher, size_of, None);
    }
}

