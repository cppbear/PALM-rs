// Answer 0

#[test]
fn test_resize_inner_minimal_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let layout = TableLayout { size: 4, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 1);

    let hasher = |table: &mut RawTableInner, _: usize| 0u64;

    unsafe {
        let result = raw_table.resize_inner(&allocator, 1, &hasher, Fallibility::Infallible, layout);
    }
}

#[test]
fn test_resize_inner_full_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let layout = TableLayout { size: 8, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    raw_table.items = 4;

    let hasher = |table: &mut RawTableInner, index: usize| index as u64;

    unsafe {
        let result = raw_table.resize_inner(&allocator, 4, &hasher, Fallibility::Infallible, layout);
    }
}

#[test]
fn test_resize_inner_non_full_buckets() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let layout = TableLayout { size: 16, ctrl_align: 8 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 8);
    raw_table.items = 3;

    let hasher = |table: &mut RawTableInner, index: usize| index as u64;

    unsafe {
        let result = raw_table.resize_inner(&allocator, 5, &hasher, Fallibility::Infallible, layout);
    }
}

#[test]
fn test_resize_inner_edge_case() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let layout = TableLayout { size: 32, ctrl_align: 16 };
    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 16);
    raw_table.items = 8;

    let hasher = |table: &mut RawTableInner, index: usize| index as u64;

    unsafe {
        let result = raw_table.resize_inner(&allocator, 8, &hasher, Fallibility::Infallible, layout);
    }
}

