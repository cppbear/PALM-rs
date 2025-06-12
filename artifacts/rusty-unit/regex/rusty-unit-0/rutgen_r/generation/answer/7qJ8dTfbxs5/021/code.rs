// Answer 0

#[test]
fn test_follow_epsilons_empty_look_start_line() {
    struct TestDFA {
        cache: Cache,
        prog: Vec<Inst>,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    #[derive(Default)]
    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    #[derive(Clone)]
    enum Inst {
        Char(u8),
        Ranges(Vec<u8>),
        Match,
        Bytes(Vec<u8>),
        EmptyLook(Box<EmptyLook>),
        Save(Box<InstPtr>),
        Split(Box<SplitInst>),
    }

    #[derive(Clone)]
    struct EmptyLook {
        look: LookType,
        goto: InstPtr,
    }

    #[derive(Clone)]
    enum LookType {
        StartLine,
        EndLine,
        StartText,
        EndText,
        WordBoundaryAscii,
        NotWordBoundaryAscii,
        WordBoundary,
        NotWordBoundary,
    }

    #[derive(Clone)]
    struct SplitInst {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    type InstPtr = usize;

    let mut cache = Cache { stack: Vec::new() };
    let mut prog = vec![
        Inst::EmptyLook(Box::new(EmptyLook { 
            look: LookType::StartLine, 
            goto: 1 
        })),
        Inst::Match,
    ];
    
    let mut dfa = TestDFA { cache, prog };

    let mut q = SparseSet::new();
    let flags = EmptyFlags { start_line: false, end_line: false, start: false, end: false, word_boundary: false, not_word_boundary: false };

    dfa.cache.stack.push(0);
    dfa.follow_epsilons(0, &mut q, flags);

    // Assert that the state was not added to the set q since the flag is false
    assert!(!q.contains(0));
}

#[test]
fn test_follow_epsilons_with_flags() {
    struct TestDFA {
        cache: Cache,
        prog: Vec<Inst>,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    #[derive(Default)]
    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    #[derive(Clone)]
    enum Inst {
        Char(u8),
        Ranges(Vec<u8>),
        Match,
        Bytes(Vec<u8>),
        EmptyLook(Box<EmptyLook>),
        Save(Box<InstPtr>),
        Split(Box<SplitInst>),
    }

    #[derive(Clone)]
    struct EmptyLook {
        look: LookType,
        goto: InstPtr,
    }

    #[derive(Clone)]
    enum LookType {
        StartLine,
        EndLine,
        StartText,
        EndText,
        WordBoundaryAscii,
        NotWordBoundaryAscii,
        WordBoundary,
        NotWordBoundary,
    }

    #[derive(Clone)]
    struct SplitInst {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    type InstPtr = usize;

    let mut cache = Cache { stack: Vec::new() };
    let mut prog = vec![
        Inst::EmptyLook(Box::new(EmptyLook { 
            look: LookType::StartLine, 
            goto: 1 
        })),
        Inst::Match,
    ];
    
    let mut dfa = TestDFA { cache, prog };

    let mut q = SparseSet::new();
    let flags = EmptyFlags { start_line: true, end_line: false, start: false, end: false, word_boundary: false, not_word_boundary: false };

    dfa.cache.stack.push(0);
    dfa.follow_epsilons(0, &mut q, flags);

    // Assert that the state was added to the set q since the flag is true
    assert!(q.contains(0));
}

#[derive(Default)]
struct SparseSet {
    data: std::collections::HashSet<usize>,
}

impl SparseSet {
    fn new() -> Self {
        SparseSet { data: std::collections::HashSet::new() }
    }
    
    fn insert(&mut self, value: usize) {
        self.data.insert(value);
    }

    fn contains(&self, value: usize) -> bool {
        self.data.contains(&value)
    }
}

