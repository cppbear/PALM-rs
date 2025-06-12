// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use hashbrown::HashMap; // Assuming the table is represented by HashMap or needs to implement a similar trait.

    struct TestTable {
        data: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestTable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let mut table = TestTable { data: HashMap::new() };
    table.data.insert(1, 10);
    table.data.insert(2, 20);
    
    let result = format!("{:?}", table);
    assert!(result.contains("10")); // Check if it contains the string "10"
    assert!(result.contains("20")); // Check if it contains the string "20"
}

#[test]
fn test_fmt_empty() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestTable {
        data: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestTable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    let table = TestTable { data: HashMap::new() };
    
    let result = format!("{:?}", table);
    assert_eq!(result, "[]"); // Check that formatting an empty table outputs an empty list
}

