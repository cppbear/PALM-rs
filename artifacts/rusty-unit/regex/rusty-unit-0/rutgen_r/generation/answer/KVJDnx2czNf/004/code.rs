// Answer 0

fn test_c_capture_success() {
    struct Hir;
    struct InstHole;
    struct Patch {
        hole: usize,
        entry: usize,
    }

    struct Compiler {
        num_exprs: usize,
        compiled: Compiled,
        insts: Vec<usize>,
    }

    struct Compiled {
        is_dfa: bool,
    }

    impl Compiler {
        fn c(&mut self, _expr: &Hir) -> Result<Patch, ()> {
            Ok(Patch { hole: 1, entry: self.insts.len() })
        }

        fn push_hole(&mut self, _hole: InstHole) -> usize {
            self.insts.push(0);
            self.insts.len() - 1
        }

        fn fill(&mut self, _hole: usize, _entry: usize) {}

        fn fill_to_next(&mut self, _hole: usize) {}
        
        fn c_capture(&mut self, first_slot: usize, expr: &Hir) -> Result<Patch, ()> {
            if self.num_exprs > 1 || self.compiled.is_dfa {
                self.c(expr)
            } else {
                let entry = self.insts.len();
                let hole = self.push_hole(InstHole);
                let patch = self.c(expr)?;
                self.fill(hole, patch.entry);
                self.fill_to_next(patch.hole);
                let hole = self.push_hole(InstHole);
                Ok(Patch { hole, entry })
            }
        }
    }

    let mut compiler = Compiler {
        num_exprs: 1,
        compiled: Compiled { is_dfa: false },
        insts: vec![],
    };

    let expr = Hir;
    let result = compiler.c_capture(0, &expr);
    
    assert!(result.is_ok());
    if let Ok(patch) = result {
        assert_eq!(patch.entry, 0);
        assert!(patch.hole >= 0);
    }
}

