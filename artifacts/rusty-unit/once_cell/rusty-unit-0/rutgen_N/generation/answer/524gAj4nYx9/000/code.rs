// Answer 0

#[derive(Debug)]
struct Cell<T>(Option<T>);

impl<T> Cell<T> {
    pub fn new() -> Self {
        Cell(None)
    }

    pub fn set(&mut self, value: T) {
        self.0 = Some(value);
    }

    pub fn get_unchecked(&self) -> &T {
        self.0.as_ref().unwrap() // this will panic if the cell is not initialized
    }
}

#[test]
fn test_get_unchecked_with_initialized_cell() {
    let mut cell = Cell::new();
    cell.set(42);
    
    unsafe {
        let value = cell.get_unchecked();
        assert_eq!(*value, 42);
    }
}

#[should_panic(expected = "None")]
#[test]
fn test_get_unchecked_with_uninitialized_cell() {
    let cell: Cell<i32> = Cell::new();
    
    unsafe {
        cell.get_unchecked(); // should panic because the cell is not initialized
    }
}

