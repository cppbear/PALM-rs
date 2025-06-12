// Answer 0

#[derive(Debug)]
struct Literals {
    limit_size: usize,
    limit_class: usize,
}

impl Literals {
    fn empty() -> Self {
        Literals {
            limit_size: 0,
            limit_class: 0,
        }
    }

    fn set_limit_size(mut self, size: usize) -> Self {
        self.limit_size = size;
        self
    }

    fn set_limit_class(mut self, class: usize) -> Self {
        self.limit_class = class;
        self
    }
}

#[derive(Debug)]
struct TestStruct {
    limit_size: usize,
    limit_class: usize,
}

impl TestStruct {
    fn to_empty(&self) -> Literals {
        let mut lits = Literals::empty();
        lits.set_limit_size(self.limit_size)
            .set_limit_class(self.limit_class);
        lits
    }
}

#[test]
fn test_to_empty() {
    let test_struct = TestStruct { limit_size: 10, limit_class: 20 };
    let result = test_struct.to_empty();
    assert_eq!(result.limit_size, 10);
    assert_eq!(result.limit_class, 20);
}

#[test]
fn test_to_empty_with_zero_values() {
    let test_struct = TestStruct { limit_size: 0, limit_class: 0 };
    let result = test_struct.to_empty();
    assert_eq!(result.limit_size, 0);
    assert_eq!(result.limit_class, 0);
}

