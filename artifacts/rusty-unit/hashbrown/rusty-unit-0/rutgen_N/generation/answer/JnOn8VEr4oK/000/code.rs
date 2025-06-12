// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAllocator;

    impl MockAllocator {
        fn new() -> Self {
            MockAllocator
        }
    }

    struct Table<A> {
        raw: RawTable<A>,
    }

    struct RawTable<A> {
        allocator: A,
    }

    impl<A> RawTable<A> {
        fn new(allocator: A) -> Self {
            RawTable { allocator }
        }

        fn allocator(&self) -> &A {
            &self.allocator
        }
    }

    impl Table<MockAllocator> {
        fn new(allocator: MockAllocator) -> Self {
            Table {
                raw: RawTable::new(allocator),
            }
        }
    }

    #[test]
    fn test_allocator() {
        let allocator = MockAllocator::new();
        let table = Table::new(allocator);
        let alloc_ref = table.allocator();
        assert!(alloc_ref.is_instance::<MockAllocator>());
    }
}

