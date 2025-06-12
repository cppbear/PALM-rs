// Answer 0

#[test]
fn test_push_hole_with_save() {
    struct TestCompiler {
        insts: Vec<MaybeInst>,
    }
    impl Compiler {
        fn new() -> Self {
            Compiler {
                insts: vec![],
                compiled: Program::new(),
                capture_name_idx: HashMap::new(),
                num_exprs: 0,
                size_limit: 10 * (1 << 20),
                suffix_cache: SuffixCache::new(1000),
                utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
                byte_classes: ByteClassSet::new(),
            }
        }
        
        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }
    
    let mut compiler = Compiler::new();
    let hole = compiler.push_hole(InstHole::Save { slot: 0 });
    assert!(matches!(hole, Hole::One(0)));
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_hole_with_empty_look() {
    struct TestCompiler {
        insts: Vec<MaybeInst>,
    }
    impl Compiler {
        fn new() -> Self {
            Compiler {
                insts: vec![],
                compiled: Program::new(),
                capture_name_idx: HashMap::new(),
                num_exprs: 0,
                size_limit: 10 * (1 << 20),
                suffix_cache: SuffixCache::new(1000),
                utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
                byte_classes: ByteClassSet::new(),
            }
        }
        
        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }

    let mut compiler = Compiler::new();
    let hole = compiler.push_hole(InstHole::EmptyLook { look: EmptyLook::new() });
    assert!(matches!(hole, Hole::One(0)));
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_hole_with_char() {
    struct TestCompiler {
        insts: Vec<MaybeInst>,
    }
    impl Compiler {
        fn new() -> Self {
            Compiler {
                insts: vec![],
                compiled: Program::new(),
                capture_name_idx: HashMap::new(),
                num_exprs: 0,
                size_limit: 10 * (1 << 20),
                suffix_cache: SuffixCache::new(1000),
                utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
                byte_classes: ByteClassSet::new(),
            }
        }
        
        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }

    let mut compiler = Compiler::new();
    let hole = compiler.push_hole(InstHole::Char { c: 'a' });
    assert!(matches!(hole, Hole::One(0)));
    assert_eq!(compiler.insts.len(), 1);
}

