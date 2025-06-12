// Answer 0

#[test]
fn test_new_cache_empty() {
    struct Program {
        byte_classes: [u8; 256],
        insts: Vec<u8>,
    }

    struct CacheInner {
        compiled: std::collections::HashMap<u8, u8>,
        trans: Transitions,
        states: Vec<u8>,
        start_states: Vec<u8>,
        stack: Vec<u8>,
        flush_count: usize,
        size: usize,
    }

    struct Cache {
        inner: CacheInner,
        qcur: SparseSet,
        qnext: SparseSet,
    }

    struct Transitions {
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Self {
            Transitions { num_byte_classes }
        }
    }

    struct SparseSet {
        set: Vec<bool>,
    }

    impl SparseSet {
        fn new(size: usize) -> Self {
            SparseSet { set: vec![false; size] }
        }
    }

    impl CacheInner {
        fn reset_size(&mut self) {
            self.size = 0;
        }
    }

    fn new(prog: &Program) -> Cache {
        let num_byte_classes = (prog.byte_classes[255] as usize + 1) + 1;
        let starts = vec![0; 256];
        let mut cache = Cache {
            inner: CacheInner {
                compiled: std::collections::HashMap::new(),
                trans: Transitions::new(num_byte_classes),
                states: vec![],
                start_states: starts,
                stack: vec![],
                flush_count: 0,
                size: 0,
            },
            qcur: SparseSet::new(prog.insts.len()),
            qnext: SparseSet::new(prog.insts.len()),
        };
        cache.inner.reset_size();
        cache
    }

    let prog = Program {
        byte_classes: [0; 256],
        insts: vec![],
    };
    let cache = new(&prog);
    
    assert_eq!(cache.inner.flush_count, 0);
    assert_eq!(cache.inner.size, 0);
    assert_eq!(cache.inner.start_states.len(), 256);
    assert!(cache.inner.states.is_empty());
    assert!(cache.inner.compiled.is_empty());
}

