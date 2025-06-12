pub fn new(prog: &Program) -> Self {
        // We add 1 to account for the special EOF byte.
        let num_byte_classes = (prog.byte_classes[255] as usize + 1) + 1;
        let starts = vec![STATE_UNKNOWN; 256];
        let mut cache = Cache {
            inner: CacheInner {
                compiled: HashMap::new(),
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