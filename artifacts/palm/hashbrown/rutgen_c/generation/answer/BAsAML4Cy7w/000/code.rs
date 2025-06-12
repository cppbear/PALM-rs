// Answer 0

#[test]
fn test_debug_vacant_entry() {
    use crate::raw::{Global, RawTable};
    use crate::{DefaultHashBuilder, HashMap};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let key = String::from("test_key");
    let value = 42;
    let hash_builder = DefaultHashBuilder;
    let mut table = RawTable::new();
    let mut hashmap = HashMap {
        hash_builder,
        table,
    };

    let vacant_entry = VacantEntry {
        hash: 0,
        key,
        table: &mut hashmap,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", vacant_entry);
    
    assert!(result.is_ok());
    assert!(output.contains("VacantEntry"));
}

