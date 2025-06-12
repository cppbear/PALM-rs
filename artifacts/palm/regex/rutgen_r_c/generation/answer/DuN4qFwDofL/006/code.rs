// Answer 0

#[test]
fn test_fill_split_many_holes_with_one_non_empty() {
    struct DummyInst;
    impl DummyInst {
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
    }

    let mut compiler = Compiler {
        insts: vec![MaybeInst::Compiled(DummyInst)],
        compiled: Program::new(),
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 10 * (1 << 20),
        suffix_cache: SuffixCache::new(1000),
        utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
        byte_classes: ByteClassSet::new(),
    };

    let hole1 = Hole::One(0);
    let hole2 = Hole::One(1);
    let hole3 = Hole::Many(vec![hole1, hole2]);

    let result = compiler.fill_split(hole3, Some(0), None);
    
    if let Hole::Many(new_holes) = result {
        assert_eq!(new_holes.len(), 1);
    } else {
        panic!("Expected Hole::Many with one non-empty hole");
    }
}

#[test]
fn test_fill_split_many_holes_with_mixed_goto() {
    struct DummyInst;
    impl DummyInst {
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
    }

    let mut compiler = Compiler {
        insts: vec![MaybeInst::Compiled(DummyInst), MaybeInst::Compiled(DummyInst)],
        compiled: Program::new(),
        capture_name_idx: HashMap::new(),
        num_exprs: 0,
        size_limit: 10 * (1 << 20),
        suffix_cache: SuffixCache::new(1000),
        utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
        byte_classes: ByteClassSet::new(),
    };

    let hole1 = Hole::One(0);
    let hole2 = Hole::One(1);
    let hole3 = Hole::Many(vec![hole1, hole2]);

    let result = compiler.fill_split(hole3, Some(0), Some(1));
    
    if let Hole::Many(new_holes) = result {
        assert_eq!(new_holes.len(), 1);
    } else {
        panic!("Expected Hole::Many with one non-empty hole");
    }
}

