// Answer 0

#[test]
fn test_follow_epsilons_with_conditions() {
    struct Cache {
        stack: Vec<InstPtr>,
    }

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

    struct Prog {
        inst: Vec<Inst>,
    }

    enum Inst {
        Char(u8),
        Ranges(Vec<(u8, u8)>),
        Match,
        Bytes(Vec<u8>),
        EmptyLook(EmptyLook),
        Save(Save),
        Split(Split),
    }

    struct EmptyLook {
        look: LookType,
        goto: u32,
    }

    struct Save {
        goto: u32,
    }

    struct Split {
        goto1: u32,
        goto2: u32,
    }

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

    struct DFA {
        cache: Cache,
        prog: Vec<Inst>,
    }

    impl DFA {
        fn new(prog: Vec<Inst>) -> Self {
            DFA {
                cache: Cache { stack: Vec::new() },
                prog,
            }
        }

        fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {
            use Inst::*;

            self.cache.stack.push(ip);
            while let Some(mut ip) = self.cache.stack.pop() {
                loop {
                    if q.contains(ip as usize) {
                        break;
                    }
                    q.insert(ip as usize);
                    match self.prog[ip as usize] {
                        Char(_) | Ranges(_) | Match | Bytes(_) => {
                            break;
                        }
                        EmptyLook(ref inst) => {
                            match inst.look {
                                LookType::NotWordBoundaryAscii if flags.not_word_boundary => {
                                    ip = inst.goto;
                                }
                                LookType::NotWordBoundary if flags.not_word_boundary => {
                                    ip = inst.goto;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        Save(ref inst) => {
                            ip = inst.goto;
                        }
                        Split(ref inst) => {
                            self.cache.stack.push(inst.goto2);
                            ip = inst.goto1;
                        }
                    }
                }
            }
        }
    }

    type InstPtr = u32;

    let inst0 = EmptyLook { look: LookType::NotWordBoundaryAscii, goto: 1 };
    let inst1 = EmptyLook { look: LookType::NotWordBoundary, goto: 2 };
    let prog = vec![Inst::EmptyLook(inst0), Inst::EmptyLook(inst1)];
    let mut dfa = DFA::new(prog);
    
    let mut q = SparseSet::new();
    let flags = EmptyFlags {
        start_line: false,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: true,
    };
    
    dfa.follow_epsilons(0, &mut q, flags);
    
    assert!(q.contains(0));
    assert!(q.contains(1));
    assert!(q.contains(2));
}

