// Answer 0

#[test]
fn test_follow_epsilons_with_empty_look_and_word_boundary() {
    struct SparseSet {
        set: Vec<bool>,
    }

    impl SparseSet {
        fn new(size: usize) -> Self {
            SparseSet {
                set: vec![false; size],
            }
        }

        fn insert(&mut self, index: usize) {
            self.set[index] = true;
        }

        fn contains(&self, index: usize) -> bool {
            self.set[index]
        }
    }

    struct InstPtr(usize);

    struct EmptyFlags {
        start_line: bool,
        end_line: bool,
        start: bool,
        end: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    struct Program {
        instructions: Vec<Inst>,
    }

    enum Inst {
        EmptyLook(EmptyLookInst),
        Match,
        Bytes,
        Char,
        Ranges,
        Save(SaveInst),
        Split(SplitInst),
    }

    struct EmptyLookInst {
        look: LookType,
        goto: InstPtr,
    }

    enum LookType {
        WordBoundaryAscii,
    }

    struct SaveInst {
        goto: InstPtr,
    }

    struct SplitInst {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    struct DFA {
        prog: Vec<Inst>,
        cache: Cache,
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    impl DFA {
        fn new(prog: Vec<Inst>) -> Self {
            DFA {
                prog,
                cache: Cache { stack: Vec::new() },
            }
        }

        fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {
            self.cache.stack.push(ip);
            while let Some(mut ip) = self.cache.stack.pop() {
                loop {
                    if q.contains(ip.0) {
                        break;
                    }
                    q.insert(ip.0);
                    match &self.prog[ip.0] {
                        Inst::EmptyLook(inst) => {
                            if inst.look == LookType::WordBoundaryAscii && flags.word_boundary {
                                ip = inst.goto;
                            } else {
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
    }

    let mut prog = vec![
        Inst::EmptyLook(EmptyLookInst {
            look: LookType::WordBoundaryAscii,
            goto: InstPtr(2),
        }),
        Inst::Match,
        Inst::EmptyLook(EmptyLookInst {
            look: LookType::WordBoundaryAscii,
            goto: InstPtr(3),
        }),
    ];

    let mut dfa = DFA::new(prog);
    let mut q = SparseSet::new(5);  // Arbitrary size
    let flags = EmptyFlags {
        start_line: false,
        end_line: false,
        start: false,
        end: false,
        word_boundary: true,
        not_word_boundary: false,
    };

    // Starting at the first instruction that is an EmptyLook
    dfa.follow_epsilons(InstPtr(0), &mut q, flags);

    // Assert that the expected states have been reached
    assert!(q.contains(0)); // Initial state should be added
    assert!(q.contains(2)); // Followed state should also be added
    assert!(!q.contains(1)); // Match shouldn't be added
    assert!(!q.contains(3)); // Second EmptyLook should not be added without re-evaluation
}

