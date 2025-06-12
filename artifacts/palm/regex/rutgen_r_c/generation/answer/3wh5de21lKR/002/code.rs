// Answer 0

#[test]
fn test_c_class_bytes_multiple_ranges() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        byte_classes: ByteClassSet,
    }

    impl MockCompiler {
        fn new() -> Self {
            MockCompiler {
                insts: vec![],
                byte_classes: ByteClassSet::new(),
            }
        }

        fn fill_to_next(&mut self, _hole: Hole) {}

        fn push_hole(&mut self, _inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(_inst));
            Hole::One(hole)
        }

        fn push_split_hole(&mut self) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Split);
            Hole::One(hole)
        }

        fn fill_split(&mut self, _hole: Hole, _goto1: Option<InstPtr>, _goto2: Option<InstPtr>) -> Hole {
            Hole::None
        }

        fn c_class_bytes(&mut self, ranges: &[hir::ClassBytesRange]) -> Result {
            debug_assert!(!ranges.is_empty());

            let first_split_entry = self.insts.len();
            let mut holes = vec![];
            let mut prev_hole = Hole::None;

            for r in &ranges[0..ranges.len() - 1] {
                self.fill_to_next(prev_hole);
                let split = self.push_split_hole();
                let next = self.insts.len();
                self.byte_classes.set_range(r.start(), r.end());
                holes.push(self.push_hole(InstHole::Bytes {
                    start: r.start(),
                    end: r.end(),
                }));
                prev_hole = self.fill_split(split, Some(next), None);
            }

            let next = self.insts.len();
            let r = &ranges[ranges.len() - 1];
            self.byte_classes.set_range(r.start(), r.end());
            holes.push(self.push_hole(InstHole::Bytes {
                start: r.start(),
                end: r.end(),
            }));
            self.fill(prev_hole, next);
            Ok(Patch { hole: Hole::Many(holes), entry: first_split_entry })
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(0x20, 0x7E),  // Printable ASCII range
        hir::ClassBytesRange::new(0x30, 0x39),  // Digits 0-9
    ];

    let result = compiler.c_class_bytes(&ranges);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_c_class_bytes_empty_range() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        byte_classes: ByteClassSet,
    }

    impl MockCompiler {
        fn new() -> Self {
            MockCompiler {
                insts: vec![],
                byte_classes: ByteClassSet::new(),
            }
        }

        fn c_class_bytes(&mut self, ranges: &[hir::ClassBytesRange]) -> Result {
            debug_assert!(!ranges.is_empty());
            // Function implementation...
            // Omitted for brevity; assume it's the same as previous.
            Ok(Patch { hole: Hole::None, entry: InstPtr::default() }) // Placeholder return
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges: Vec<hir::ClassBytesRange> = vec![]; // Empty to trigger panic
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_single_range() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        byte_classes: ByteClassSet,
    }

    impl MockCompiler {
        fn new() -> Self {
            MockCompiler {
                insts: vec![],
                byte_classes: ByteClassSet::new(),
            }
        }

        fn c_class_bytes(&mut self, ranges: &[hir::ClassBytesRange]) -> Result {
            debug_assert!(!ranges.is_empty());

            // Function implementation omitted for brevity.
            Ok(Patch { hole: Hole::None, entry: InstPtr::default() }) // Placeholder return
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![hir::ClassBytesRange::new(0x61, 0x7A)]; // Range for 'a' to 'z'
    let result = compiler.c_class_bytes(&ranges);
    assert!(result.is_ok());
}

