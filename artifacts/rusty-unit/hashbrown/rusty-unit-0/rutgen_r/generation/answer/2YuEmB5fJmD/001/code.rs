// Answer 0

#[test]
fn test_new_empty_hash_table() {
    use std::ptr::NonNull;

    struct Group;

    impl Group {
        const fn static_empty() -> &'static [u8] {
            &[]
        }
    }

    struct HashTable {
        ctrl: NonNull<u8>,
        bucket_mask: usize,
        items: usize,
        growth_left: usize,
    }

    impl HashTable {
        const fn new() -> Self {
            Self {
                ctrl: unsafe {
                    NonNull::new_unchecked(Group::static_empty().as_ptr().cast_mut().cast())
                },
                bucket_mask: 0,
                items: 0,
                growth_left: 0,
            }
        }
    }

    let table = HashTable::new();
    assert_eq!(table.bucket_mask, 0);
    assert_eq!(table.items, 0);
    assert_eq!(table.growth_left, 0);
}

