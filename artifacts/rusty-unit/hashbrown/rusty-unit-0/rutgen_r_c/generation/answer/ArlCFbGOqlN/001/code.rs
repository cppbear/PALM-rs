// Answer 0

#[test]
fn test_record_item_insert_at_with_empty_tag() {
    let mut table_inner = RawTableInner {
        bucket_mask: 3,
        ctrl: NonNull::new_unchecked((&mut [0u8; 16] as *mut _).cast()), // Example NonNull pointer
        growth_left: 1,
        items: 0,
    };
    
    let old_ctrl = Tag::EMPTY; // Represents an empty tag
    unsafe {
        table_inner.record_item_insert_at(0, old_ctrl, 42); // Use a hash of 42
    }

    assert_eq!(table_inner.growth_left, 1); // growth_left should not change
    assert_eq!(table_inner.items, 1); // items should increment to 1
}

#[test]
fn test_record_item_insert_at_with_deleted_tag() {
    let mut table_inner = RawTableInner {
        bucket_mask: 3,
        ctrl: NonNull::new_unchecked((&mut [0u8; 16] as *mut _).cast()),
        growth_left: 1,
        items: 0,
    };

    let old_ctrl = Tag::DELETED; // Represents a deleted tag
    unsafe {
        table_inner.record_item_insert_at(1, old_ctrl, 13); // Use a hash of 13
    }
    
    assert_eq!(table_inner.growth_left, 0); // growth_left should decrease by 1
    assert_eq!(table_inner.items, 1); // items should increment to 1
}

#[test]
fn test_record_item_insert_at_multiple_calls() {
    let mut table_inner = RawTableInner {
        bucket_mask: 3,
        ctrl: NonNull::new_unchecked((&mut [0u8; 16] as *mut _).cast()),
        growth_left: 2,
        items: 0,
    };

    let old_ctrl = Tag::DELETED; // Represent the deleted tag initially
    unsafe {
        table_inner.record_item_insert_at(2, old_ctrl, 99); // First call
    }
    
    assert_eq!(table_inner.growth_left, 1); // growth_left decreases by 1
    assert_eq!(table_inner.items, 1); // First item added

    let new_old_ctrl = Tag::EMPTY; // Now using an empty tag
    unsafe {
        table_inner.record_item_insert_at(3, new_old_ctrl, 100); // Second call
    }

    assert_eq!(table_inner.growth_left, 1); // growth_left doesn't change with EMPTY
    assert_eq!(table_inner.items, 2); // Second item added
}

