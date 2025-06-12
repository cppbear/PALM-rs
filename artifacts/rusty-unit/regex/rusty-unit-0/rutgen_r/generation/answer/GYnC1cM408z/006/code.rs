// Answer 0

fn test_compile_many() {
    struct Hir {
        anchored_start: bool,
        anchored_end: bool,
    }
    
    impl Hir {
        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }
        
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    struct Patch {
        hole: Hole,
        entry: usize,
    }

    struct Hole;

    struct CompileContext {
        compiled: CompiledData,
        insts: Vec<()>,
    }

    struct CompiledData {
        is_anchored_start: bool,
        is_anchored_end: bool,
        matches: Vec<usize>,
        start: usize,
    }

    impl CompileContext {
        fn new() -> Self {
            Self {
                compiled: CompiledData {
                    is_anchored_start: false,
                    is_anchored_end: false,
                    matches: Vec::new(),
                    start: 0,
                },
                insts: Vec::new(),
            }
        }

        fn needs_dotstar(&self) -> bool {
            true
        }

        fn c_dotstar(&mut self) -> Result<Patch, ()> {
            Ok(Patch { hole: Hole, entry: 1 })
        }

        fn fill_to_next(&mut self, _hole: Hole) {}

        fn push_split_hole(&mut self) -> Hole {
            Hole
        }

        fn c_capture(&mut self, _arg: usize, _expr: &Hir) -> Result<Patch, ()> {
            Ok(Patch { hole: Hole, entry: 2 })
        }

        fn fill_split(&mut self, _split: Hole, _entry: Option<usize>, _none: Option<()>) -> Hole {
            Hole
        }

        fn push_compiled(&mut self, _inst: ()) {}

        fn fill(&mut self, _prev_hole: Hole, _entry: usize) {}

        fn compile_finish(&mut self) -> Result<(), ()> {
            Ok(())
        }
    }

    let mut context = CompileContext::new();
    let expr1 = Hir { anchored_start: true, anchored_end: true };
    let expr2 = Hir { anchored_start: true, anchored_end: false };

    let exprs = vec![expr1, expr2];
    
    let _ = context.compile_many(&exprs).unwrap();
}

impl CompileContext {
    fn compile_many(
        &mut self,
        exprs: &[Hir],
    ) -> Result<(), ()> {
        debug_assert!(exprs.len() > 1);

        self.compiled.is_anchored_start =
            exprs.iter().all(|e| e.is_anchored_start());
        self.compiled.is_anchored_end =
            exprs.iter().all(|e| e.is_anchored_end());
        
        let mut dotstar_patch = Patch { hole: Hole, entry: 0 };
        if self.needs_dotstar() {
            dotstar_patch = self.c_dotstar()?;
            self.compiled.start = dotstar_patch.entry;
        } else {
            self.compiled.start = 0; 
        }
        self.fill_to_next(dotstar_patch.hole);

        let mut prev_hole = Hole;
        for (i, expr) in exprs[0..exprs.len() - 1].iter().enumerate() {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let patch = self.c_capture(0, expr)?;
            self.fill_to_next(patch.hole);
            self.compiled.matches.push(self.insts.len());
            self.push_compiled(());
            prev_hole = self.fill_split(split, Some(patch.entry), None);
        }
        let i = exprs.len() - 1;
        let patch = self.c_capture(0, &exprs[i])?;
        self.fill(prev_hole, patch.entry);
        self.fill_to_next(patch.hole);
        self.compiled.matches.push(self.insts.len());
        self.push_compiled(());
        self.compile_finish()?;
        Ok(())
    }
}

