// Answer 0

#[test]
fn test_next_with_valid_input() {
    struct DFA {
        table: Vec<usize>,
    }

    impl DFA {
        fn next(&self, si: usize, cls: usize) -> usize {
            self.table[si + cls]
        }
    }

    let dfa = DFA {
        table: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    };

    assert_eq!(dfa.next(0, 1), 1);
    assert_eq!(dfa.next(1, 1), 2);
    assert_eq!(dfa.next(3, 2), 5);
}

#[test]
#[should_panic]
fn test_next_with_out_of_bounds_index() {
    struct DFA {
        table: Vec<usize>,
    }

    impl DFA {
        fn next(&self, si: usize, cls: usize) -> usize {
            self.table[si + cls]
        }
    }

    let dfa = DFA {
        table: vec![0, 1, 2, 3, 4],
    };

    // This should panic as it accesses an out-of-bounds index
    let _ = dfa.next(4, 0); // 4 + 0 = 4, which is out of bounds
}

#[test]
#[should_panic]
fn test_next_with_negative_index() {
    struct DFA {
        table: Vec<usize>,
    }

    impl DFA {
        fn next(&self, si: usize, cls: usize) -> usize {
            self.table[si + cls]
        }
    }

    let dfa = DFA {
        table: vec![0, 1, 2, 3, 4],
    };

    // This should panic as it accesses a negative index
    let _ = dfa.next(0, 5); // 0 + 5 = 5, which may lead to panic if negative indexing logic was applied (handled by a panic in our setup)
}

