// Answer 0

#[derive(Debug)]
struct Group {
    // This structure defines the static empty representation of our hash table.
}

impl Group {
    const fn static_empty() -> &'static [u8] {
        &[]
    }
}

struct HashTable {
    ctrl: std::ptr::NonNull<u8>,
    bucket_mask: usize,
    items: usize,
    growth_left: usize,
}

impl HashTable {
    const fn new() -> Self {
        Self {
            ctrl: unsafe {
                std::ptr::NonNull::new_unchecked(Group::static_empty().as_ptr().cast_mut().cast())
            },
            bucket_mask: 0,
            items: 0,
            growth_left: 0,
        }
    }
}

#[test]
fn test_new_hash_table() {
    let table = HashTable::new();
    assert_eq!(table.bucket_mask, 0);
    assert_eq!(table.items, 0);
    assert_eq!(table.growth_left, 0);
}

#[test]
fn test_new_hash_table_is_non_null() {
    let table = HashTable::new();
    assert!(!table.ctrl.as_ptr().is_null());
}

