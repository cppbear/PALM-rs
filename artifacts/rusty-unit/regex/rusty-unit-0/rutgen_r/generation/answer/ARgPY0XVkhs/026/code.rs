// Answer 0

#[derive(Debug)]
struct Literals {
    literals: Vec<String>,
}

impl Literals {
    fn literals(&self) -> &Vec<String> {
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
impl BoyerMooreSearch {
    fn should_use(_: &[u8]) -> bool {
        true
    }

    fn new(_: Vec<u8>) -> Self {
        BoyerMooreSearch
    }
}

#[derive(Debug)]
struct FreqyPacked;

impl FreqyPacked {
    fn new(_: Vec<u8>) -> Self {
        FreqyPacked
    }
}

#[derive(Debug)]
struct TeddyAVX2;

impl TeddyAVX2 {
    fn available() -> bool {
        false
    }

    fn new(_: &Literals) -> Option<Self> {
        None
    }
}

#[derive(Debug)]
struct TeddySSSE3;

impl TeddySSSE3 {
    fn available() -> bool {
        false
    }

    fn new(_: &Literals) -> Option<Self> {
        None
    }
}

#[derive(Debug)]
struct AcAutomaton;

impl AcAutomaton {
    fn new(_: Vec<String>) -> Self {
        AcAutomaton
    }

    fn into_full(self) -> Self {
        self
    }
}

fn new(lits: &Literals, sset: SingleByteSet) -> Matcher {
    if lits.literals().is_empty() {
        return Matcher::Empty;
    }
    if sset.dense.len() >= 26 {
        return Matcher::Empty;
    }
    if sset.complete {
        return Matcher::Bytes(sset);
    }
    if lits.literals().len() == 1 {
        let lit = lits.literals()[0].as_bytes().to_vec();
        if BoyerMooreSearch::should_use(lit.as_slice()) {
            return Matcher::BoyerMoore(BoyerMooreSearch::new(lit));
        } else {
            return Matcher::FreqyPacked(FreqyPacked::new(lit));
        }
    }
    let is_aho_corasick_fast = sset.dense.len() == 1 && sset.all_ascii;
    if TeddyAVX2::available() && !is_aho_corasick_fast {
        const MAX_TEDDY_LITERALS: usize = 32;
        if lits.literals().len() <= MAX_TEDDY_LITERALS {
            if let Some(ted) = TeddyAVX2::new(lits) {
                return Matcher::TeddyAVX2(ted);
            }
        }
    }
    if TeddySSSE3::available() && !is_aho_corasick_fast {
        const MAX_TEDDY_LITERALS: usize = 32;
        if lits.literals().len() <= MAX_TEDDY_LITERALS {
            if let Some(ted) = TeddySSSE3::new(lits) {
                return Matcher::TeddySSSE3(ted);
            }
        }
    }
    let pats = lits.literals().to_owned();
    Matcher::AC(AcAutomaton::new(pats).into_full())
}

#[test]
fn test_new_single_lit() {
    let lits = Literals {
        literals: vec!["test".to_string()],
    };
    
    let sset = SingleByteSet {
        dense: vec![1, 2], // less than 26 elements
        complete: false,
        all_ascii: true,
    };
    
    let matcher = new(&lits, sset);
    
    match matcher {
        Matcher::AC(_) => {},
        _ => panic!("Expected Matcher::AC(_)"),
    }
}

#[test]
fn test_new_multiple_literals() {
    let lits = Literals {
        literals: vec!["first".to_string(), "second".to_string()],
    };

    let sset = SingleByteSet {
        dense: vec![1, 2], // less than 26 elements
        complete: false,
        all_ascii: false,
    };
    
    let matcher = new(&lits, sset);
    
    match matcher {
        Matcher::AC(_) => {},
        _ => panic!("Expected Matcher::AC(_)"),
    }
}

