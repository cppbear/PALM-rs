// Answer 0

#[derive(Default)]
struct TestTable<T> {
    items: Vec<T>,
}

impl<T> TestTable<T> {
    pub fn new() -> Self {
        TestTable {
            items: Vec::new(),
        }
    }
    
    pub fn clear(&mut self) {
        if self.is_empty() {
            return;
        }
        let mut self_ = Guard::new(self);
        unsafe {
            self_.table.drop_elements::<T>();
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

struct Guard<'a, T> {
    table: &'a mut TestTable<T>,
}

impl<'a, T> Guard<'a, T> {
    pub fn new(table: &'a mut TestTable<T>) -> Self {
        Guard { table }
    }
}

// Test cases
#[test]
fn test_clear_empty_table() {
    let mut table: TestTable<i32> = TestTable::new();
    table.clear();
    assert!(table.is_empty());
}

#[test]
fn test_clear_non_empty_table() {
    let mut table: TestTable<i32> = TestTable::new();
    table.items.push(1);
    table.items.push(2);
    table.clear();
    assert!(table.is_empty());
}

#[test]
fn test_clear_multiple_clears() {
    let mut table: TestTable<i32> = TestTable::new();
    table.items.push(1);
    table.clear();
    assert!(table.is_empty());
    table.clear(); // calling clear again on an already empty table
    assert!(table.is_empty());
}

