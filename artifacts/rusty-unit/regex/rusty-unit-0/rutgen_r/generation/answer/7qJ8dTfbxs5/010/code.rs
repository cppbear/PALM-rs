// Answer 0

#[test]
fn test_follow_epsilons_start_line() {
    struct SparseSet {
        set: std::collections::HashSet<usize>,
    }

    impl SparseSet {
        fn new() -> Self {
            SparseSet {
                set: std::collections::HashSet::new(),
            }
        }

        fn contains(&self, value: usize) -> bool {
            self.set.contains(&value)
        }

        fn insert(&mut self, value: usize) {
            self.set.insert(value);
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

    struct InstPtr(usize);

    struct Program {
        instructions: Vec<Inst>,
    }

    impl Program {
        fn new(instructions: Vec<Inst>) -> Self {
            Program { instructions }
        }
    }

    enum Inst {
        EmptyLook(EmptyLook),
        Match,
        Bytes,
        Char(u8),
        Ranges,
        Save(SaveInst),
        Split(SplitInst),
    }

    struct EmptyLook {
        look: LookKind,
        goto: u32,
    }

    enum LookKind {
        StartLine,
        NotWordBoundary,
        WordBoundary,
    }

    struct SaveInst {
        goto: u32,
    }

    struct SplitInst {
        goto1: u32,
        goto2: u32,
    }

    struct DFA {
        prog: Program,
        cache: Cache,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    impl DFA {
        fn new(prog: Program) -> Self {
            Self {
                prog,
                cache: Cache { stack: Vec::new() },
            }
        }

        fn follow_epsilons(
            &mut self,
            ip: InstPtr,
            q: &mut SparseSet,
            flags: EmptyFlags,
        ) {
            // Function implementation goes here (omitted here to focus on tests)
        }
    }

    let mut q = SparseSet::new();
    let instructions = vec![
        Inst::EmptyLook(EmptyLook { look: LookKind::StartLine, goto: 1 }),
        Inst::EmptyLook(EmptyLook { look: LookKind::NotWordBoundary, goto: 2 }),
        Inst::Match,
    ];

    let prog = Program::new(instructions);
    let mut dfa = DFA::new(prog);

    let flags = EmptyFlags {
        start_line: true,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: true,
    };

    dfa.cache.stack.push(InstPtr(0));
    dfa.follow_epsilons(InstPtr(0), &mut q, flags);

    assert!(q.contains(0));
    assert!(q.contains(1));
    assert!(q.contains(2));
}

#[test]
fn test_follow_epsilons_word_boundary() {
    struct SparseSet {
        set: std::collections::HashSet<usize>,
    }

    impl SparseSet {
        fn new() -> Self {
            SparseSet {
                set: std::collections::HashSet::new(),
            }
        }

        fn contains(&self, value: usize) -> bool {
            self.set.contains(&value)
        }

        fn insert(&mut self, value: usize) {
            self.set.insert(value);
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

    struct InstPtr(usize);

    struct Program {
        instructions: Vec<Inst>,
    }

    impl Program {
        fn new(instructions: Vec<Inst>) -> Self {
            Program { instructions }
        }
    }

    enum Inst {
        EmptyLook(EmptyLook),
        Match,
        Bytes,
        Char(u8),
        Ranges,
        Save(SaveInst),
        Split(SplitInst),
    }

    struct EmptyLook {
        look: LookKind,
        goto: u32,
    }

    enum LookKind {
        StartLine,
        WordBoundary,
        NotWordBoundary,
    }

    struct SaveInst {
        goto: u32,
    }

    struct SplitInst {
        goto1: u32,
        goto2: u32,
    }

    struct DFA {
        prog: Program,
        cache: Cache,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    impl DFA {
        fn new(prog: Program) -> Self {
            Self {
                prog,
                cache: Cache { stack: Vec::new() },
            }
        }

        fn follow_epsilons(
            &mut self,
            ip: InstPtr,
            q: &mut SparseSet,
            flags: EmptyFlags,
        ) {
            // Function implementation goes here (omitted here to focus on tests)
        }
    }

    let mut q = SparseSet::new();
    let instructions = vec![
        Inst::EmptyLook(EmptyLook { look: LookKind::WordBoundary, goto: 1 }),
        Inst::EmptyLook(EmptyLook { look: LookKind::NotWordBoundary, goto: 2 }),
        Inst::Match,
    ];

    let prog = Program::new(instructions);
    let mut dfa = DFA::new(prog);

    let flags = EmptyFlags {
        start_line: false,
        end_line: false,
        start: false,
        end: false,
        word_boundary: true,
        not_word_boundary: true,
    };

    dfa.cache.stack.push(InstPtr(0));
    dfa.follow_epsilons(InstPtr(0), &mut q, flags);

    assert!(q.contains(0));
    assert!(q.contains(1));
}

