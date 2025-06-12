// Answer 0

#[test]
fn test_into_iter_from_valid() {
    let allocator = Global;
    let mut table = RawTable::with_capacity_in(8, allocator);
    // Assuming we insert 4 items
    unsafe {
        table.insert(1, "item1", |s| s.len() as u64);
        table.insert(2, "item2", |s| s.len() as u64);
        table.insert(3, "item3", |s| s.len() as u64);
        table.insert(4, "item4", |s| s.len() as u64);
    }

    let iter = unsafe { table.iter() };
    let into_iter = unsafe { table.into_iter_from(iter) };

    let capacity = table.capacity();
    assert!(into_iter.iter.items <= capacity);
}

#[test]
fn test_into_iter_from_empty() {
    let allocator = Global;
    let table = RawTable::<&str>::with_capacity_in(4, allocator);
    let iter = unsafe { table.iter() };
    let into_iter = unsafe { table.into_iter_from(iter) };
    
    assert!(into_iter.iter.items == 0);
}

#[test]
fn test_into_iter_from_single_item() {
    let allocator = Global;
    let mut table = RawTable::with_capacity_in(4, allocator);
    // Inserting a single item
    unsafe {
        table.insert(1, "item1", |s| s.len() as u64);
    }

    let iter = unsafe { table.iter() };
    let into_iter = unsafe { table.into_iter_from(iter) };

    assert!(into_iter.iter.items == 1);
}

#[should_panic]
fn test_into_iter_from_invalid_start() {
    let allocator = Global;
    let mut table = RawTable::<&str>::with_capacity_in(4, allocator);
    // Inserting a couple of items
    unsafe {
        table.insert(1, "item1", |s| s.len() as u64);
        table.insert(2, "item2", |s| s.len() as u64);
    }

    // Create an iterator without covering all items
    let iter = unsafe { table.iter() };

    // Attempting to directly call into_iter_from should trigger a panic
    let _into_iter = unsafe { table.into_iter_from(iter) };
}

#[test]
fn test_into_iter_from_all_items() {
    let allocator = Global;
    let mut table = RawTable::with_capacity_in(8, allocator);
    // Inserting multiple items
    unsafe {
        table.insert(1, "item1", |s| s.len() as u64);
        table.insert(2, "item2", |s| s.len() as u64);
        table.insert(3, "item3", |s| s.len() as u64);
        table.insert(4, "item4", |s| s.len() as u64);
    }

    // Assuming we create an iterator that correctly covers all items
    let iter = unsafe { table.iter() };

    // Call into_iter_from with a proper iterator
    let into_iter = unsafe { table.into_iter_from(iter) };

    // Verify that the allocation is not None since we inserted items
    assert!(into_iter.allocation.is_some());
}

