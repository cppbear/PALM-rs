// Answer 0

#[test]
fn test_set_next_valid_transition() {
    struct TestDFA {
        table: Vec<u32>,
    }

    impl TestDFA {
        fn new(size: usize) -> Self {
            Self { table: vec![0; size] }
        }

        fn set_next(&mut self, si: usize, cls: usize, next: usize) {
            self.table[si + cls] = next as u32;
        }
    }

    let mut dfa = TestDFA::new(10);
    dfa.set_next(0, 0, 1);
    assert_eq!(dfa.table[0], 1);
}

#[test]
#[should_panic]
fn test_set_next_si_out_of_bounds() {
    struct TestDFA {
        table: Vec<u32>,
    }

    impl TestDFA {
        fn new(size: usize) -> Self {
            Self { table: vec![0; size] }
        }

        fn set_next(&mut self, si: usize, cls: usize, next: usize) {
            self.table[si + cls] = next as u32;
        }
    }

    let mut dfa = TestDFA::new(10);
    dfa.set_next(10, 0, 1); // si is out of bounds
}

#[test]
#[should_panic]
fn test_set_next_cls_out_of_bounds() {
    struct TestDFA {
        table: Vec<u32>,
    }

    impl TestDFA {
        fn new(size: usize) -> Self {
            Self { table: vec![0; size] }
        }

        fn set_next(&mut self, si: usize, cls: usize, next: usize) {
            self.table[si + cls] = next as u32;
        }
    }

    let mut dfa = TestDFA::new(10);
    dfa.set_next(0, 10, 1); // cls is out of bounds
}

#[test]
fn test_set_next_boundary_conditions() {
    struct TestDFA {
        table: Vec<u32>,
    }

    impl TestDFA {
        fn new(size: usize) -> Self {
            Self { table: vec![0; size] }
        }

        fn set_next(&mut self, si: usize, cls: usize, next: usize) {
            self.table[si + cls] = next as u32;
        }
    }

    let mut dfa = TestDFA::new(10);
    dfa.set_next(0, 0, 1);
    assert_eq!(dfa.table[0], 1);
    dfa.set_next(0, 9, 2);
    assert_eq!(dfa.table[9], 2);
}

