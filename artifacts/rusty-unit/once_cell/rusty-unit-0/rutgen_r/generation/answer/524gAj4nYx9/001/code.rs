// Answer 0

#[derive(Debug)]
struct MyCell<T>(Option<T>);

impl<T> MyCell<T> {
    fn new() -> Self {
        MyCell(None)
    }
    
    fn initialize(&mut self, value: T) {
        self.0 = Some(value);
    }
    
    unsafe fn get_unchecked(&self) -> &T {
        self.0.as_ref().unwrap().get_unchecked()
    }
}

#[test]
fn test_get_unchecked_initialized() {
    let mut cell = MyCell::new();
    cell.initialize(42);
    
    unsafe {
        let value: &i32 = cell.get_unchecked();
        assert_eq!(*value, 42);
    }
}

#[should_panic]
#[test]
fn test_get_unchecked_uninitialized() {
    let cell = MyCell::new();
    
    unsafe {
        cell.get_unchecked(); // Should panic as the cell is uninitialized
    }
}

