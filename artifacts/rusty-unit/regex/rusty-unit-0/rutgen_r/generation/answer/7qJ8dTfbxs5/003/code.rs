// Answer 0

#[test]
fn test_follow_epsilons_with_empty_flags() {
    struct SparseSet {
        set: Vec<usize>,
    }

    impl SparseSet {
        fn new() -> Self {
            SparseSet { set: Vec::new() }
        }

        fn contains(&self, index: usize) -> bool {
            self.set.contains(&index)
        }

        fn insert(&mut self, index: usize) {
            if !self.contains(index) {
                self.set.push(index);
            }
        }
    }

    struct InstPtr(usize);

    struct DummyProg {
        prog: Vec<ProgInst>,
    }

    enum ProgInst {
        Bytes(u8),
        Match(),
    }

    struct Cache {
        stack: Vec<InstPtr>,
    }

    struct DummyState {
        cache: Cache,
        prog: Vec<ProgInst>,
    }

    impl DummyState {
        fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {
            // Follow the original function's content
            let mut current_ip = ip;
            self.cache.stack.push(current_ip);
            while let Some(mut ip) = self.cache.stack.pop() {
                loop {
                    if q.contains(ip.0) {
                        break;
                    }
                    q.insert(ip.0);
                    match self.prog[ip.0] {
                        ProgInst::Bytes(_) | ProgInst::Match() => {
                            break;
                        }
                    }
                }
            }
        }
    }

    struct EmptyFlags {
        start: bool,
        end: bool,
        start_line: bool,
        end_line: bool,
        word_boundary: bool,
        not_word_boundary: bool,
    }

    let prog = vec![ProgInst::Bytes(1), ProgInst::Match(), ProgInst::Bytes(2)];
    let mut cache = Cache { stack: Vec::new() };
    let mut state = DummyState { cache, prog };
    let mut q = SparseSet::new();
    let ip = InstPtr(0); // This will point to the first Bytes instruction.

    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    state.follow_epsilons(ip, &mut q, flags);

    assert!(q.contains(0)); // we expect that starting with the first instruction gets inserted
    assert!(q.contains(1)); // following the current instruction should lead to the next Bytes or Match
    assert_eq!(q.set.len(), 2); // only 0 and 1 should be added
}

