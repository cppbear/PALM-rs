// Answer 0

#[test]
fn test_prepare_rehash_in_place() {
    use crate::alloc::Global;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {}

    let table_layout = TableLayout::default(); // Replace with actual default constructor if available
    let capacity = 16; // Sample capacity
    let fallibility = Fallibility::Infallible;

    let mut raw_table = RawTableInner::with_capacity(&MockAllocator, table_layout, capacity);

    // Simulate the initial state of control bytes
    for i in 0..raw_table.buckets() {
        unsafe {
            let tag_to_set = if i % 4 == 0 {
                Tag::FULL // Assume alternate indices are FULL
            } else {
                Tag::DELETED // All others are DELETED
            };
            raw_table.set_ctrl(i, tag_to_set);
        }
    }

    unsafe {
        raw_table.prepare_rehash_in_place();
    }

    for i in 0..raw_table.buckets() {
        unsafe {
            let tag = raw_table.ctrl(i).read();
            if i % 4 == 0 {
                assert_eq!(tag, Tag::DELETED, "Expected DELETED for FULL original tag at index {}", i);
            } else {
                assert_eq!(tag, Tag::EMPTY, "Expected EMPTY for DELETED original tag at index {}", i);
            }
        }
    }
}

#[test]
#[should_panic]
fn test_prepare_rehash_in_place_uninitialized() {
    let mut raw_table = RawTableInner {
        bucket_mask: 0,
        ctrl: NonNull::dangling(),
        growth_left: 0,
        items: 0,
    };

    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

