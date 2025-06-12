// Answer 0

#[test]
fn test_into_table() {
    struct TestAllocator;
    
    struct TestTable {
        raw: RawTable<i32, TestAllocator>,
    }
    
    impl TestTable {
        fn new() -> Self {
            TestTable {
                raw: RawTable::new(),
            }
        }
    }

    struct VacantEntryTest<'a> {
        hash: u64,
        insert_slot: InsertSlot,
        table: &'a mut TestTable,
    }
    
    impl<'a> VacantEntry<'a, i32, TestAllocator> {
        fn new(insert_slot: InsertSlot, table: &'a mut TestTable) -> Self {
            VacantEntry {
                hash: 0,
                insert_slot,
                table,
            }
        }
    }

    let mut test_table = TestTable::new();
    let insert_slot = InsertSlot { index: 0 };
    let vacant_entry = VacantEntryTest::new(insert_slot, &mut test_table);

    let table_ref = vacant_entry.into_table();
    
    assert_eq!(table_ref as *mut _, &mut test_table as *mut _);
}

