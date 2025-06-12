// Answer 0

#[test]
fn test_allocator() {
    struct TestAllocator;

    impl TestAllocator {
        fn allocator(&self) -> &Self {
            self
        }
    }

    struct Raw {
        allocator: TestAllocator,
    }

    struct Table {
        raw: Raw,
    }

    let allocator = TestAllocator;
    let raw = Raw { allocator };
    let table = Table { raw };

    assert_eq!(std::ptr::eq(table.allocator(), &table.raw.allocator), true);
}

#[test]
fn test_allocator_with_different_allocators() {
    struct Allocator1;
    struct Allocator2;

    impl Allocator1 {
        fn allocator(&self) -> &Self {
            self
        }
    }

    impl Allocator2 {
        fn allocator(&self) -> &Self {
            self
        }
    }

    struct Raw1 {
        allocator: Allocator1,
    }

    struct Raw2 {
        allocator: Allocator2,
    }

    struct Table1 {
        raw: Raw1,
    }

    struct Table2 {
        raw: Raw2,
    }

    let allocator1 = Allocator1;
    let raw1 = Raw1 { allocator: allocator1 };
    let table1 = Table1 { raw: raw1 };

    let allocator2 = Allocator2;
    let raw2 = Raw2 { allocator: allocator2 };
    let table2 = Table2 { raw: raw2 };

    assert_ne!(std::ptr::eq(table1.allocator(), table2.allocator()), true);
}

