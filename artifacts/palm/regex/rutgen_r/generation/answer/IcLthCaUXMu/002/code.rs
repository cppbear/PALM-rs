// Answer 0

#[test]
fn test_class_exceeds_limits_empty_literals() {
    struct DummyLit {
        length: usize,
    }

    impl DummyLit {
        fn len(&self) -> usize {
            self.length
        }

        fn is_cut(&self) -> bool {
            false
        }
    }

    struct TestStruct {
        lits: Vec<DummyLit>,
        limit_class: usize,
        limit_size: usize,
    }

    impl TestStruct {
        fn class_exceeds_limits(&self, size: usize) -> bool {
            if size > self.limit_class {
                return true;
            }
            let new_byte_count =
                if self.lits.is_empty() {
                    size
                } else {
                    self.lits
                        .iter()
                        .fold(0, |accum, lit| {
                            accum + if lit.is_cut() {
                                0
                            } else {
                                (lit.len() + 1) * size
                            }
                        })
                };
            new_byte_count > self.limit_size
        }
    }

    // Initialize test conditions
    let limit_class = 5;
    let limit_size = 5;
    let size = limit_class; // size == self.limit_class
    let lits: Vec<DummyLit> = Vec::new(); // self.lits is empty

    let test_struct = TestStruct {
        lits,
        limit_class,
        limit_size,
    };

    // Run the test
    assert!(test_struct.class_exceeds_limits(size));
}

#[test]
fn test_class_exceeds_limits_non_empty_literals() {
    struct DummyLit {
        length: usize,
    }

    impl DummyLit {
        fn len(&self) -> usize {
            self.length
        }

        fn is_cut(&self) -> bool {
            false
        }
    }

    struct TestStruct {
        lits: Vec<DummyLit>,
        limit_class: usize,
        limit_size: usize,
    }

    impl TestStruct {
        fn class_exceeds_limits(&self, size: usize) -> bool {
            if size > self.limit_class {
                return true;
            }
            let new_byte_count =
                if self.lits.is_empty() {
                    size
                } else {
                    self.lits
                        .iter()
                        .fold(0, |accum, lit| {
                            accum + if lit.is_cut() {
                                0
                            } else {
                                (lit.len() + 1) * size
                            }
                        })
                };
            new_byte_count > self.limit_size
        }
    }

    // Initialize test conditions
    let limit_class = 5;
    let limit_size = 10; // setting limit_size to enable overflow in calculation
    let size = limit_class; // size == self.limit_class
    let lits = vec![DummyLit { length: 1 }]; // self.lits is non-empty

    let test_struct = TestStruct {
        lits,
        limit_class,
        limit_size,
    };

    // Run the test
    assert!(test_struct.class_exceeds_limits(size));
}

