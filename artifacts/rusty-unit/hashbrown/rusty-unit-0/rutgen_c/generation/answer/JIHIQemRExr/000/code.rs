// Answer 0

#[test]
fn test_fix_insert_slot_empty_bucket() {
    struct TestAllocator;
    
    let mut table = RawTableInner::with_capacity(&TestAllocator, TableLayout::default(), 4);
    
    // Simulate control bytes indicating an empty bucket at index 0
    unsafe {
        *table.ctrl(0).cast::<Tag>() = Tag(0); // Tag::EMPTY
    }

    let insert_slot = unsafe { table.fix_insert_slot(0) };
    assert_eq!(insert_slot.index, 0);
}

#[test]
fn test_fix_insert_slot_full_bucket() {
    struct TestAllocator;

    let mut table = RawTableInner::with_capacity(&TestAllocator, TableLayout::default(), 4);

    // Simulate control bytes indicating a full bucket at index 0
    unsafe {
        *table.ctrl(0).cast::<Tag>() = Tag(1); // Tag::FULL
        *table.ctrl(1).cast::<Tag>() = Tag(0); // Tag::EMPTY
    }

    let insert_slot = unsafe { table.fix_insert_slot(0) };
    assert_eq!(insert_slot.index, 1); // It should find the next available slot
}

#[test]
fn test_fix_insert_slot_boundary_condition() {
    struct TestAllocator;

    let mut table = RawTableInner::with_capacity(&TestAllocator, TableLayout::default(), 4);

    // Simulate all buckets being full
    unsafe {
        *table.ctrl(0).cast::<Tag>() = Tag(1); // Tag::FULL
        *table.ctrl(1).cast::<Tag>() = Tag(1); // Tag::FULL
        *table.ctrl(2).cast::<Tag>() = Tag(1); // Tag::FULL
        *table.ctrl(3).cast::<Tag>() = Tag(0); // Tag::EMPTY
    }

    let insert_slot = unsafe { table.fix_insert_slot(2) };
    assert_eq!(insert_slot.index, 3); // It should find the next available slot
}

