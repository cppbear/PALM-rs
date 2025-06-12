// Answer 0

#[test]
fn test_follow_epsilons_split() {
    struct MockDFA {
        cache: Cache,
        prog: Vec<prog::Inst>,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    struct SparseSet {
        elements: std::collections::HashSet<usize>,
    }

    impl SparseSet {
        fn new() -> Self {
            SparseSet {
                elements: std::collections::HashSet::new(),
            }
        }

        fn contains(&self, value: usize) -> bool {
            self.elements.contains(&value)
        }

        fn insert(&mut self, value: usize) {
            self.elements.insert(value);
        }
    }

    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    type InstPtr = usize;

    // Create an instance of MockDFA with a simple program.
    let mut dfa = MockDFA {
        cache: Cache {
            stack: vec![0], // Initial state
        },
        prog: vec![
            prog::Inst::Split(prog::SplitInst { goto1: 1, goto2: 2 }),
            prog::Inst::Match(prog::MatchInst {}),
            prog::Inst::Match(prog::MatchInst {}),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags {
        start_line: false,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    dfa.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0)); // Should contain the initial state
    assert!(q.contains(1)); // Should follow the Split to goto1
    assert!(!q.contains(2)); // Should not contain goto2 since it's not followed
}

#[test]
fn test_follow_epsilons_no_split() {
    struct MockDFA {
        cache: Cache,
        prog: Vec<prog::Inst>,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    struct SparseSet {
        elements: std::collections::HashSet<usize>,
    }

    impl SparseSet {
        fn new() -> Self {
            SparseSet {
                elements: std::collections::HashSet::new(),
            }
        }

        fn contains(&self, value: usize) -> bool {
            self.elements.contains(&value)
        }

        fn insert(&mut self, value: usize) {
            self.elements.insert(value);
        }
    }

    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    type InstPtr = usize;

    // Create an instance of MockDFA with no Split instructions.
    let mut dfa = MockDFA {
        cache: Cache {
            stack: vec![0], // Initial state
        },
        prog: vec![
            prog::Inst::Match(prog::MatchInst {}),
            prog::Inst::Match(prog::MatchInst {}),
        ],
    };

    let mut q = SparseSet::new();
    let flags = EmptyFlags {
        start_line: false,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    dfa.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0)); // Should contain the initial state
    assert!(!q.contains(1)); // Should not contain any other state without split
}

