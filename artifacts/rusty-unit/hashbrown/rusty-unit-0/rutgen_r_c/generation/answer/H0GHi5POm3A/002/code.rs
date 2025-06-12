// Answer 0

#[test]
fn test_get_non_existent_key() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let result = map.get(&2);
    assert_eq!(result, None);
}

