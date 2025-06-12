// Answer 0

#[derive(Debug)]
struct Literals {
    literals: Vec<Vec<u8>>,
}

impl Literals {
    fn literals(&self) -> &[Vec<u8>] {
        &self.literals
    }
}

#[derive(Debug)]
struct SingleByteSet {
    dense: Vec<u8>,
    complete: bool,
    all_ascii: bool,
}

#[derive(Debug)]
enum Matcher {
    Empty,
    Bytes(SingleByteSet),
    BoyerMoore(BoyerMooreSearch),
    FreqyPacked(FreqyPacked),
    TeddyAVX2(TeddyAVX2),
    TeddySSSE3(TeddySSSE3),
    AC(AcAutomaton),
}

#[derive(Debug)]
struct BoyerMooreSearch;

#[derive(Debug)]
struct FreqyPacked;

#[derive(Debug)]
struct TeddyAVX2;

#[derive(Debug)]
struct TeddySSSE3;

#[derive(Debug)]
struct AcAutomaton;

impl AcAutomaton {
    fn new(_: Vec<Vec<u8>>) -> Self {
        AcAutomaton
    }

    fn into_full(self) -> Matcher {
        Matcher::AC(self)
    }
}

#[test]
fn test_new_with_empty_return() {
    let lits = Literals { literals: vec![] };
    let sset = SingleByteSet {
        dense: vec![1; 26], // 26 elements
        complete: false,
        all_ascii: false,
    };

    let matcher = new(&lits, sset);
    match matcher {
        Matcher::Empty => (),
        _ => panic!("Expected Matcher::Empty"),
    }
}

#[test]
fn test_new_with_dense_limit() {
    let lits = Literals { literals: vec![b"test".to_vec()] };
    let sset = SingleByteSet {
        dense: vec![1; 26], // exactly 26 elements
        complete: false,
        all_ascii: false,
    };

    let matcher = new(&lits, sset);
    match matcher {
        Matcher::Empty => (),
        _ => panic!("Expected Matcher::Empty"),
    }
}

