// Answer 0

#[test]
fn test_allocator() {
    struct TestAllocator;
    
    impl TestAllocator {
        fn new() -> Self {
            TestAllocator
        }
    }
    
    struct TestTable<A> {
        allocator: A,
    }
    
    impl<A> TestTable<A> {
        fn new(allocator: A) -> Self {
            TestTable { allocator }
        }
        
        fn allocator(&self) -> &A {
            &self.allocator
        }
    }
    
    struct TestStruct<A> {
        table: TestTable<A>,
    }
    
    impl<A> TestStruct<A> {
        fn new(table: TestTable<A>) -> Self {
            TestStruct { table }
        }
        
        pub fn allocator(&self) -> &A {
            self.table.allocator()
        }
    }
    
    let allocator = TestAllocator::new();
    let table = TestTable::new(allocator);
    let test_struct = TestStruct::new(table);
    
    let alloc_ref = test_struct.allocator();
    assert_eq!(alloc_ref as *const TestAllocator, &test_struct.table.allocator as *const _);
}

