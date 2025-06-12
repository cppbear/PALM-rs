// Answer 0

fn test_c_class_single_char() {
    struct TestCompiler {
        compiled: Program,
        insts: Vec<MaybeInst>,
    }

    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                compiled: Program::new(),
                insts: vec![],
            }
        }

        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }

    let mut compiler = TestCompiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![hir::ClassUnicodeRange::new('a', 'a')]; // This will meet the conditions.
    let result = compiler.c_class(&ranges);

    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0)); // The hole should be created at index 0.
            assert_eq!(patch.entry, 0); // The entry should be the same as the current length of insts.
        },
        _ => panic!("Expected Ok but got an error."),
    }
}

fn test_c_class_multiple_chars() {
    struct TestCompiler {
        compiled: Program,
        insts: Vec<MaybeInst>,
    }

    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                compiled: Program::new(),
                insts: vec![],
            }
        }

        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }

    let mut compiler = TestCompiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![
        hir::ClassUnicodeRange::new('a', 'd'), // Example range from 'a' to 'd'
    ];
    let result = compiler.c_class(&ranges);

    match result {
        Ok(patch) => {
            assert!(matches!(patch.hole, Hole::One(_))); // Ensure hole is created.
            assert_eq!(patch.entry, 0); // The entry should point to the length of insts
        },
        _ => panic!("Expected Ok but got an error."),
    }
}

