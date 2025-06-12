// Answer 0

#[test]
fn test_get_or_init_empty_cell() {
    struct Cell {
        value: Option<Box<i32>>,
    }

    impl Cell {
        pub fn new() -> Self {
            Cell { value: None }
        }

        pub fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut cell = Cell::new();
    let value = cell.get_or_init(|| Box::new(42));
    assert_eq!(*value, 42);
}

#[test]
fn test_get_or_init_non_empty_cell() {
    struct Cell {
        value: Option<Box<i32>>,
    }
    
    impl Cell {
        pub fn new() -> Self {
            Cell { value: None }
        }
        
        pub fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut cell = Cell::new();
    cell.get_or_init(|| Box::new(42));
    let value = cell.get_or_init(|| Box::new(100));
    assert_eq!(*value, 42);
}

#[test]
fn test_get_or_init_with_multiple_calls() {
    struct Cell {
        value: Option<Box<i32>>,
    }
    
    impl Cell {
        pub fn new() -> Self {
            Cell { value: None }
        }

        pub fn get_or_init<F>(&mut self, f: F) -> &Box<i32>
        where
            F: FnOnce() -> Box<i32>,
        {
            if self.value.is_none() {
                self.value = Some(f());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut cell = Cell::new();
    let value1 = cell.get_or_init(|| Box::new(42));
    let value2 = cell.get_or_init(|| Box::new(100));
    assert_eq!(*value1, 42);
    assert_eq!(value1, value2);
}

