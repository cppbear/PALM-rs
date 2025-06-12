// Answer 0

#[test]
fn test_vacant_entry_fmt() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut table = HashTable::<u32, MockAllocator> {
        raw: RawTable::new(),
    };

    let hash: u64 = 0; // Lower boundary
    let insert_slot = InsertSlot { index: 0 }; // Lower boundary
    let vacant_entry = VacantEntry { hash, insert_slot, table: &mut table };

    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), format_args!("{:?}", vacant_entry));
}

#[test]
fn test_vacant_entry_fmt_upper_bound_hash() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut table = HashTable::<u32, MockAllocator> {
        raw: RawTable::new(),
    };

    let hash: u64 = u64::MAX; // Upper boundary
    let insert_slot = InsertSlot { index: 0 }; // Lower boundary
    let vacant_entry = VacantEntry { hash, insert_slot, table: &mut table };

    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), format_args!("{:?}", vacant_entry));
}

#[test]
fn test_vacant_entry_fmt_large_index() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut table = HashTable::<u32, MockAllocator> {
        raw: RawTable::new(),
    };

    let hash: u64 = 1;
    let insert_slot = InsertSlot { index: usize::MAX }; // Upper boundary
    let vacant_entry = VacantEntry { hash, insert_slot, table: &mut table };

    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), format_args!("{:?}", vacant_entry));
}

