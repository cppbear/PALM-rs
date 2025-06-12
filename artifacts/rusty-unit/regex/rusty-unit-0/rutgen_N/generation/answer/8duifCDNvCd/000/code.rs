// Answer 0

#[derive(Default)]
struct Cache {
    size: usize,
}

#[derive(Default)]
struct Prog {
    size: usize,
}

impl Prog {
    fn approximate_size(&self) -> usize {
        self.size
    }
}

struct DFA {
    cache: Cache,
    prog: Prog,
}

impl DFA {
    fn approximate_size(&self) -> usize {
        self.cache.size + self.prog.approximate_size()
    }
}

#[test]
fn test_approximate_size_zero() {
    let dfa = DFA::default();
    assert_eq!(dfa.approximate_size(), 0);
}

#[test]
fn test_approximate_size_non_zero_cache() {
    let dfa = DFA {
        cache: Cache { size: 10 },
        prog: Prog::default(),
    };
    assert_eq!(dfa.approximate_size(), 10);
}

#[test]
fn test_approximate_size_non_zero_prog() {
    let dfa = DFA {
        cache: Cache::default(),
        prog: Prog { size: 5 },
    };
    assert_eq!(dfa.approximate_size(), 5);
}

#[test]
fn test_approximate_size_non_zero_both() {
    let dfa = DFA {
        cache: Cache { size: 10 },
        prog: Prog { size: 5 },
    };
    assert_eq!(dfa.approximate_size(), 15);
}

