// Answer 0

#[derive(Default)]
struct Cache {
    size: usize,
}

#[derive(Default)]
struct Program {
    size: usize,
}

impl Program {
    fn approximate_size(&self) -> usize {
        self.size
    }
}

struct DFA {
    cache: Cache,
    prog: Program,
}

impl DFA {
    fn approximate_size(&self) -> usize {
        self.cache.size + self.prog.approximate_size()
    }
}

#[test]
fn test_approximate_size_zero() {
    let dfa = DFA {
        cache: Cache { size: 0 },
        prog: Program { size: 0 },
    };
    assert_eq!(dfa.approximate_size(), 0);
}

#[test]
fn test_approximate_size_positive_values() {
    let dfa = DFA {
        cache: Cache { size: 10 },
        prog: Program { size: 20 },
    };
    assert_eq!(dfa.approximate_size(), 30);
}

#[test]
fn test_approximate_size_large_values() {
    let dfa = DFA {
        cache: Cache { size: usize::MAX / 2 },
        prog: Program { size: usize::MAX / 2 },
    };
    assert_eq!(dfa.approximate_size(), usize::MAX - 1);
}

#[test]
fn test_approximate_size_only_cache() {
    let dfa = DFA {
        cache: Cache { size: 15 },
        prog: Program { size: 0 },
    };
    assert_eq!(dfa.approximate_size(), 15);
}

#[test]
fn test_approximate_size_only_program() {
    let dfa = DFA {
        cache: Cache { size: 0 },
        prog: Program { size: 25 },
    };
    assert_eq!(dfa.approximate_size(), 25);
}

