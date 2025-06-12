// Answer 0

#[test]
fn test_c_class_bytes_single_range() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        byte_classes: ByteClassSet,
    }

    impl MockCompiler {
        fn new() -> Self {
            Self {
                insts: vec![],
                byte_classes: ByteClassSet::new(),
            }
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

        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }

        fn push_split_hole(&mut self) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Split);
            Hole::One(hole)
        }

        fn fill_to_next(&mut self, hole: Hole) {
            let next = self.insts.len();
            self.fill(hole, next);
        }

        fn fill_split(&mut self, hole: Hole, goto1: Option<InstPtr>, goto2: Option<InstPtr>) -> Hole {
            start.fill_split(hole, goto1, goto2)
        }

        fn fill(&mut self, hole: Hole, goto: InstPtr) {
            // Dummy fill implementation
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![hir::ClassBytesRange::new(0u8, 10u8)];
    let result = compiler.c_class_bytes(&ranges);
    assert!(result.is_ok());
}

#[test]
fn test_c_class_bytes_multiple_ranges() {
    struct MockCompiler {
        insts: Vec<MaybeInst>,
        byte_classes: ByteClassSet,
    }

    impl MockCompiler {
        fn new() -> Self {
            Self {
                insts: vec![],
                byte_classes: ByteClassSet::new(),
            }
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

        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }

        fn push_split_hole(&mut self) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Split);
            Hole::One(hole)
        }

        fn fill_to_next(&mut self, hole: Hole) {
            let next = self.insts.len();
            self.fill(hole, next);
        }

        fn fill_split(&mut self, hole: Hole, goto1: Option<InstPtr>, goto2: Option<InstPtr>) -> Hole {
            start.fill_split(hole, goto1, goto2)
        }

        fn fill(&mut self, hole: Hole, goto: InstPtr) {
            // Dummy fill implementation
        }
    }

    let mut compiler = MockCompiler::new();
    let ranges = vec![hir::ClassBytesRange::new(0u8, 10u8), hir::ClassBytesRange::new(20u8, 30u8)];
    let result = compiler.c_class_bytes(&ranges);
    assert!(result.is_ok());
}

