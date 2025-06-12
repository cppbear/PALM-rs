// Answer 0


#[derive(Debug)]
struct OnceCell<T> {
    value: Option<T>,
}

impl<T> OnceCell<T> {
    fn new() -> Self {
        OnceCell { value: None }
    }
    
    fn get(&self) -> &Option<T> {
        &self.value
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for OnceCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.get() {
            Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
            None => f.write_str("OnceCell(Uninit)"),
        }
    }
}

#[test]
fn test_once_cell_uninitialized() {
    let cell: OnceCell<i32> = OnceCell::new();
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| cell.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "OnceCell(Uninit)");
}


