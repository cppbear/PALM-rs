// Answer 0

#[test]
fn test_rehash_in_place_empty_table() {
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 0);
    let hasher = |_: &mut RawTableInner, _: usize| 0; // Simple hash function
    let size_of = std::mem::size_of::<u8>(); // Size for u8
    let drop = None; // No drop function for empty table
    unsafe { raw_table.rehash_in_place(&hasher, size_of, drop) };
}

#[test]
fn test_rehash_in_place_single_deleted() {
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 1);
    let hasher = |_: &mut RawTableInner, _: usize| 0; // Simple hash function
    let size_of = std::mem::size_of::<u8>(); // Size for u8
    let drop = None; // No drop function
    unsafe {
        raw_table.ctrl(0).write_bytes(Tag::DELETED.0, 1); // Simulate deleted
        raw_table.items = 1; // Set items to 1
        raw_table.rehash_in_place(&hasher, size_of, drop);
    }
}

#[test]
fn test_rehash_in_place_multiple_deleted() {
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 4);
    let hasher = |_: &mut RawTableInner, _: usize| 0; // Simple hash function
    let size_of = std::mem::size_of::<u8>(); // Size for u8
    let drop = None; // No drop function
    unsafe {
        for i in 0..4 {
            raw_table.ctrl(i).write_bytes(Tag::DELETED.0, 1); // Simulate deleted
        }
        raw_table.items = 4; // Set items to 4
        raw_table.rehash_in_place(&hasher, size_of, drop);
    }
}

#[test]
#[should_panic]
fn test_rehash_in_place_panic() {
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 1);
    let hasher = |_: &mut RawTableInner, _| panic!("Hash function panicked");
    let size_of = std::mem::size_of::<u8>(); // Size for u8
    let drop = None; // No drop function
    unsafe {
        raw_table.ctrl(0).write_bytes(Tag::DELETED.0, 1); // Simulate deleted
        raw_table.items = 1; // Set items to 1
        raw_table.rehash_in_place(&hasher, size_of, drop);
    }
}

