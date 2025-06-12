// Answer 0

#[test]
#[should_panic]
fn test_compile_many_single_expression_panics() {
    struct DummyCompiler {
        compiled: CompiledData,
        insts: Vec<Inst>,
    }
    
    struct CompiledData {
        is_anchored_start: bool,
        is_anchored_end: bool,
        start: usize,
        matches: Vec<usize>,
    }

    struct Hir {
        // dummy data
    }

    struct Patch {
        hole: Hole,
        entry: usize,
    }

    struct Hole;

    enum Inst {
        Match(usize),
    }

    impl DummyCompiler {
        fn needs_dotstar(&self) -> bool {
            // dummy implementation for testing purposes
            false
        }

        fn c_dotstar(&mut self) -> result::Result<Patch, ()> {
            // dummy implementation for testing purposes
            Ok(Patch { hole: Hole, entry: 0 })
        }

        fn c_capture(&mut self, _: usize, _: &Hir) -> result::Result<Patch, ()> {
            // dummy implementation for testing purposes
            Ok(Patch { hole: Hole, entry: 1 })
        }

        fn fill_to_next(&mut self, _: Hole) {
            // dummy implementation for testing purposes
        }

        fn push_split_hole(&mut self) -> Hole {
            // dummy implementation for testing purposes
            Hole
        }

        fn fill_split(&mut self, _: Hole, _: Option<usize>, _: Option<usize>) -> Hole {
            // dummy implementation for testing purposes
            Hole
        }

        fn push_compiled(&mut self, _: Inst) {
            // dummy implementation for testing purposes
        }

        fn compile_finish(&mut self) -> result::Result<Program, ()> {
            // dummy implementation for testing purposes
            Ok(Program)
        }

        fn compile_many(mut self, exprs: &[Hir]) -> result::Result<Program, ()> {
            debug_assert!(exprs.len() > 1);

            self.compiled.is_anchored_start =
                exprs.iter().all(|e| e.is_anchored_start());
            self.compiled.is_anchored_end =
                exprs.iter().all(|e| e.is_anchored_end());
            let mut dotstar_patch = Patch { hole: Hole::None, entry: 0 };
            if self.compiled.needs_dotstar() {
                dotstar_patch = self.c_dotstar()?;
                self.compiled.start = dotstar_patch.entry;
            } else {
                self.compiled.start = 0; // first instruction is always split
            }
            self.fill_to_next(dotstar_patch.hole);

            let mut prev_hole = Hole::None;
            for (i, expr) in exprs[0..exprs.len() - 1].iter().enumerate() {
                self.fill_to_next(prev_hole);
                let split = self.push_split_hole();
                let Patch { hole, entry } = self.c_capture(0, expr)?;
                self.fill_to_next(hole);
                self.compiled.matches.push(self.insts.len());
                self.push_compiled(Inst::Match(i));
                prev_hole = self.fill_split(split, Some(entry), None);
            }
            let i = exprs.len() - 1;
            let Patch { hole, entry } = self.c_capture(0, &exprs[i])?;
            self.fill(prev_hole, entry);
            self.fill_to_next(hole);
            self.compiled.matches.push(self.insts.len());
            self.push_compiled(Inst::Match(i));
            self.compile_finish()
        }
    }

    struct Program;

    let compiler = DummyCompiler {
        compiled: CompiledData {
            is_anchored_start: false,
            is_anchored_end: false,
            start: 0,
            matches: Vec::new(),
        },
        insts: Vec::new(),
    };

    let exprs = vec![Hir {}]; // only one expression, which should trigger a panic
    compiler.compile_many(&exprs).unwrap();
}

