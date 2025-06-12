// Answer 0

#[derive(Default)]
struct Literals {
    limit_class: usize,
}

impl Literals {
    pub fn set_limit_class(&mut self, size: usize) -> &mut Literals {
        self.limit_class = size;
        self
    }
}

#[test]
fn test_set_limit_class_zero() {
    let mut literals = Literals::default();
    literals.set_limit_class(0);
    assert_eq!(literals.limit_class, 0);
}

#[test]
fn test_set_limit_class_positive() {
    let mut literals = Literals::default();
    literals.set_limit_class(5);
    assert_eq!(literals.limit_class, 5);
}

#[test]
fn test_set_limit_class_increase_limit() {
    let mut literals = Literals::default();
    literals.set_limit_class(3);
    literals.set_limit_class(7);
    assert_eq!(literals.limit_class, 7);
}

