// Answer 0

#[test]
fn test_once_cell_uninitialized() {
    use std::fmt;

    struct OnceCell {
        value: Option<i32>,
    }

    impl OnceCell {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
    }

    impl fmt::Debug for OnceCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let cell = OnceCell::new();
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(Uninit)");
}

#[test]
fn test_once_cell_initialized() {
    use std::fmt;

    struct OnceCell {
        value: Option<i32>,
    }

    impl OnceCell {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn set(&mut self, value: i32) {
            self.value = Some(value);
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
    }

    impl fmt::Debug for OnceCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let mut cell = OnceCell::new();
    cell.set(42);
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(42)");
}

