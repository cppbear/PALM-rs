// Answer 0

#[test]
fn test_is_empty_on_empty_table() {
    struct Table {
        elements: Vec<i32>,
    }

    impl Table {
        fn len(&self) -> usize {
            self.elements.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let table = Table { elements: Vec::new() };
    assert!(table.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_table() {
    struct Table {
        elements: Vec<i32>,
    }

    impl Table {
        fn len(&self) -> usize {
            self.elements.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let table = Table { elements: vec![1, 2, 3] };
    assert!(!table.is_empty());
}

#[test]
fn test_is_empty_on_table_with_zero_elements() {
    struct Table {
        elements: Vec<i32>,
    }

    impl Table {
        fn len(&self) -> usize {
            self.elements.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let table = Table { elements: Vec::with_capacity(0) };
    assert!(table.is_empty());
}

#[test]
fn test_is_empty_on_table_with_capacity() {
    struct Table {
        elements: Vec<i32>,
    }

    impl Table {
        fn len(&self) -> usize {
            self.elements.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut table = Table { elements: Vec::with_capacity(10) };
    assert!(table.is_empty());
    table.elements.push(5);
    assert!(!table.is_empty());
}

