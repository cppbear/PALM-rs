// Answer 0

#[test]
fn test_set_full_cell() {
    struct TestCell {
        cell: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                cell: OnceCell::new(),
            }
        }

        fn set_value(&self, value: i32) -> Result<(), i32> {
            self.cell.set(value)
        }

        fn get(&self) -> Option<&i32> {
            self.cell.get()
        }
    }

    let cell = TestCell::new();
    // First, we insert a value to make the cell "full".
    let first_set_result = cell.set_value(42);
    assert_eq!(first_set_result, Ok(()));
    assert_eq!(cell.get(), Some(&42));

    // Now, we attempt to set another value; this should return Err(value).
    let second_set_result = cell.set_value(99);
    assert_eq!(second_set_result, Err(99));
    assert_eq!(cell.get(), Some(&42));
}

