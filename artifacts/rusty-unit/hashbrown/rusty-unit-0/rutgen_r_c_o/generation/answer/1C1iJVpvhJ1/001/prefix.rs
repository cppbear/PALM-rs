// Answer 0

#[test]
fn test_iter_with_default_allocator() {
    let values: Vec<(u32, u32)> = (1..=10).map(|i| (i, i)).collect();
    let table = RawTable::from_iter(values.iter());
    let iter = table.iter();
}

#[test]
fn test_iter_with_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values: Vec<(u8, u8)> = (1..=5).map(|i| (i, i * 2)).collect();
    let table = RawTable::from_iter(values.iter());
    let iter = table.iter();
}

#[should_panic]
fn test_iter_with_empty_table() {
    let empty_table: RawTable<(u32, u32)> = RawTable::new();
    let iter = empty_table.iter();
}

#[test]
fn test_iter_with_varied_sizes() {
    let values: Vec<(i32, i32)> = (1..=10).map(|i| (i * 2, i * 3)).collect();
    let table = RawTable::from_iter(values.iter());
    let iter = table.iter();

    let custom_size_allocator: CustomAllocator = CustomAllocator;
    let values_custom: Vec<(i64, i64)> = (1..=5).map(|i| (i * 4, i * 5)).collect();
    let table_custom = RawTable::from_iter(values_custom.iter());
    let iter_custom = table_custom.iter();
}

