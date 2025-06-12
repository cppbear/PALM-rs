// Answer 0

#[test]
fn test_follow_epsilons() {
    struct TestProgram {
        insts: Vec<Inst>,
    }

    impl TestProgram {
        fn new() -> Self {
            Self {
                insts: vec![
                    Inst::Save(InstSave { goto: 1, slot: 0 }),
                    Inst::Match(0),
                    Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::StartLine }),
                    Inst::EmptyLook(InstEmptyLook { goto: 3, look: EmptyLook::EndLine }),
                ],
            }
        }
    }

    struct TestCacheInner {
        stack: Vec<InstPtr>,
    }

    struct TestFsm<'a> {
        prog: &'a TestProgram,
        cache: &'a mut TestCacheInner,
    }

    impl<'a> TestFsm<'a> {
        fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {
            q.insert(ip as usize);
            self.cache.stack.push(ip);
            self.cache.stack.push(0); // To ensure `Some(mut ip)` is true for the first loop
            while let Some(mut ip) = self.cache.stack.pop() {
                loop {
                    if q.contains(ip as usize) {
                        break;
                    }
                    q.insert(ip as usize);
                    match self.prog.insts[ip as usize] {
                        Inst::Save(ref inst) => {
                            ip = inst.goto;
                        }
                        Inst::Match(_) => {
                            break; // End of epsilon following
                        }
                        Inst::EmptyLook(ref inst) => {
                            if flags.start_line {
                                ip = inst.goto;
                            } else {
                                break; // Do not follow if flags do not match
                            }
                        }
                        _ => break, // Other conditions
                    }
                }
            }
        }
    }

    let mut cache = TestCacheInner { stack: Vec::new() };
    let program = TestProgram::new();
    let mut fsm = TestFsm { prog: &program, cache: &mut cache };
    
    let mut sparse_set = SparseSet::new(10);
    let flags = EmptyFlags { start_line: true, end_line: false, start: false, end: false, word_boundary: false, not_word_boundary: false };
    
    fsm.follow_epsilons(0, &mut sparse_set, flags);
    
    assert!(sparse_set.contains(0));
    assert!(sparse_set.contains(1));
    assert!(sparse_set.contains(2)); // Should follow to the EmptyLook because of the flag
    assert!(sparse_set.len() == 3);
}

