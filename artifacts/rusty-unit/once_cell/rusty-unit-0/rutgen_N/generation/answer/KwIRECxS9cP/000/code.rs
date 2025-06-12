// Answer 0

#[derive(Default)]
struct Cell<T> {
    value: Option<Box<T>>,
}

impl<T> Cell<T> {
    pub fn get(&self) -> Option<&T> {
        self.value.as_deref()
    }
    
    pub fn init<F, E>(&mut self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<Box<T>, E>,
    {
        let box_value = f()?;
        self.value = Some(box_value);
        self.get().ok_or_else(|| panic!("Failed to initialize"))
    }
}

impl<T> Cell<T> {
    pub fn get_or_try_init<F, E>(&mut self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<Box<T>, E>,
    {
        match self.get() {
            Some(val) => Ok(val),
            None => self.init(f)
        }
    }
}

#[test]
fn test_get_or_try_init_when_empty() {
    let mut cell: Cell<i32> = Cell::default();
    let result = cell.get_or_try_init(|| Ok(Box::new(42)));
    
    assert_eq!(result, Ok(&42));
}

#[test]
fn test_get_or_try_init_when_not_empty() {
    let mut cell: Cell<i32> = Cell::default();
    let _ = cell.get_or_try_init(|| Ok(Box::new(42)));
    let result = cell.get_or_try_init(|| Ok(Box::new(43)));
    
    assert_eq!(result, Ok(&42));
}

#[test]
fn test_get_or_try_init_with_error() {
    let mut cell: Cell<String> = Cell::default();
    let result: Result<&String, &str> = cell.get_or_try_init(|| Err("Initialization failed"));
    
    assert!(result.is_err());
    assert_eq!(result, Err("Initialization failed"));
}

