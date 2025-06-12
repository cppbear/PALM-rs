// Answer 0

#[derive(Default)]
struct Imp<T> {
    value: Option<T>,
}

pub struct OnceCell<T>(Imp<T>);

impl<T> Imp<T> {
    pub const fn new() -> Self {
        Imp { value: None }
    }
}

#[test]
fn test_once_cell_new() {
    let cell: OnceCell<i32> = new();
    // Assuming we would have additional methods to validate the state of the cell.
    // For now, we are just testing that the cell can be created.
    // You would typically validate internal state here if methods were available.
}

#[test]
fn test_once_cell_new_empty() {
    let cell: OnceCell<String> = new();
    // Add assertions if there were methods to check the internal state.
}

