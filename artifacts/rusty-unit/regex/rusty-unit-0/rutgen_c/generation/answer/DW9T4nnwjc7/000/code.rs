// Answer 0

#[test]
fn test_set_limit_class() {
    struct TestHir;
    
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    // Setting the limit class to a new value
    literals.set_limit_class(3);
    assert_eq!(literals.limit_class, 3);
}

#[test]
fn test_set_limit_class_zero() {
    struct TestHir;

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    // Setting the limit class to zero
    literals.set_limit_class(0);
    assert_eq!(literals.limit_class, 0);
}

#[test]
fn test_set_limit_class_increase() {
    struct TestHir;

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    // Increasing the limit class
    literals.set_limit_class(10);
    assert_eq!(literals.limit_class, 10);
}

#[test]
fn test_set_limit_class_decrease() {
    struct TestHir;

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    // Decreasing the limit class
    literals.set_limit_class(2);
    assert_eq!(literals.limit_class, 2);
}

#[test]
fn test_set_limit_class_no_change() {
    struct TestHir;

    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 10,
        limit_class: 5,
    };

    // Setting the limit class to the same value
    literals.set_limit_class(5);
    assert_eq!(literals.limit_class, 5);
}

