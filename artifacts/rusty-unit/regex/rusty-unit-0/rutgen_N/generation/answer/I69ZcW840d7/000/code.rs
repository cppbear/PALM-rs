// Answer 0

#[test]
fn test_next() {
    struct DFA {
        table: Vec<usize>,
    }

    impl DFA {
        fn next(&self, si: usize, cls: usize) -> usize {
            self.table[si + cls]
        }
    }

    let dfa = DFA {
        table: vec![0, 1, 2, 3, 4, 5],
    };

    // Test a regular case
    let result = dfa.next(1, 2);
    assert_eq!(result, 3);

    // Test boundary case (start of the table)
    let result_boundary_start = dfa.next(0, 0);
    assert_eq!(result_boundary_start, 0);

    // Test boundary case (end of the table within bounds)
    let result_boundary_end = dfa.next(3, 1);
    assert_eq!(result_boundary_end, 4);

    // Test out-of-bounds case (should panic)
    let result_out_of_bounds = std::panic::catch_unwind(|| {
        dfa.next(5, 0);
    });
    assert!(result_out_of_bounds.is_err());
}

