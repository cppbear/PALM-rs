// Answer 0

#[test]
fn test_allocator() {
    struct MockAllocator;
    struct MockMap {
        allocator: MockAllocator,
    }

    impl MockMap {
        fn allocator(&self) -> &MockAllocator {
            &self.allocator
        }
    }

    struct TestStruct<A> {
        map: MockMap,
    }

    let mock_allocator = MockAllocator;
    let mock_map = MockMap {
        allocator: mock_allocator,
    };
    
    let test_struct = TestStruct { map: mock_map };

    assert_eq!(test_struct.allocator() as *const _, &mock_allocator as *const _);
}

