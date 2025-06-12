// Answer 0

#[test]
fn test_clear_no_drop() {
    struct TestTable {
        table: Vec<Option<i32>>,
    }

    impl TestTable {
        fn new(size: usize) -> Self {
            TestTable {
                table: vec![Some(0); size],
            }
        }

        fn clear_no_drop(&mut self) {
            for bucket in &mut self.table {
                *bucket = None;
            }
        }
    }

    let mut test_table = TestTable::new(5);
    test_table.clear_no_drop();

    assert!(test_table.table.iter().all(|bucket| bucket.is_none()));
}

#[test]
fn test_clear_no_drop_empty() {
    struct TestTable {
        table: Vec<Option<i32>>,
    }

    impl TestTable {
        fn new(size: usize) -> Self {
            TestTable {
                table: vec![None; size],
            }
        }

        fn clear_no_drop(&mut self) {
            for bucket in &mut self.table {
                *bucket = None;
            }
        }
    }

    let mut test_table = TestTable::new(0);
    test_table.clear_no_drop();

    assert!(test_table.table.is_empty());
}

