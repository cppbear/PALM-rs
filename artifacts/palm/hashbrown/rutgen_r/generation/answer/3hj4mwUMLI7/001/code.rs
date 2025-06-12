// Answer 0

#[test]
fn test_allocation_size_or_zero_empty_singleton() {
    struct StubTableLayout;
    
    struct StubHashTable {
        empty_singleton: bool,
    }

    impl StubHashTable {
        fn is_empty_singleton(&self) -> bool {
            self.empty_singleton
        }

        unsafe fn allocation_info(&self, _layout: StubTableLayout) -> (usize, Size) {
            (0, Size(0))
        }
    }

    struct Size(usize);

    let table_layout = StubTableLayout;
    let hash_table = StubHashTable { empty_singleton: true };

    unsafe {
        let size = hash_table.allocation_size_or_zero(table_layout);
        assert_eq!(size, 0);
    }
}

