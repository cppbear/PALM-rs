// Answer 0

#[test]
fn test_c_alternate_with_empty_sub_expression() {
    struct Hir;
    struct Hole;
    struct Patch {
        hole: Hole,
        entry: usize,
    }

    struct Compiler {
        insts: Vec<u8>,
    }

    impl Compiler {
        fn new() -> Self {
            Self { insts: vec![] }
        }
        
        fn fill_to_next(&mut self, _hole: Hole) {}
        
        fn push_split_hole(&mut self) -> Hole {
            Hole {}
        }

        fn c(&mut self, _e: &Hir) -> Result<Patch, Error> {
            // Simulate a case where `self.insts.len()` does not change
            Ok(Patch { hole: Hole {}, entry: self.insts.len() })
        }
        
        fn fill(&mut self, _prev_hole: Hole, _entry: usize) {}
        
        fn c_alternate(&mut self, exprs: &[Hir]) -> Result<Patch, Error> {
            debug_assert!(
                exprs.len() >= 2, "alternates must have at least 2 exprs");

            let first_split_entry = self.insts.len();
            let mut holes = vec![];
            let mut prev_hole = Hole::None;
            for e in &exprs[0..exprs.len() - 1] {
                self.fill_to_next(prev_hole);
                let split = self.push_split_hole();
                let prev_entry = self.insts.len();
                let Patch { hole, entry } = self.c(e)?;
                if prev_entry == self.insts.len() {
                    return Err(Error::Syntax(
                        "alternations cannot currently contain \
                         empty sub-expressions".to_string()));
                }
                holes.push(hole);
                prev_hole = self.fill_split(split, Some(entry), None);
            }
            let prev_entry = self.insts.len();
            let Patch { hole, entry } = self.c(&exprs[exprs.len() - 1])?;
            if prev_entry == self.insts.len() {
                return Err(Error::Syntax(
                    "alternations cannot currently contain \
                     empty sub-expressions".to_string()));
            }
            holes.push(hole);
            self.fill(prev_hole, entry);
            Ok(Patch { hole: Hole::Many(holes), entry: first_split_entry })
        }
    }

    enum Error {
        Syntax(String),
    }

    type Result<T> = std::result::Result<T, Error>;

    let mut compiler = Compiler::new();
    let exprs = vec![Hir, Hir]; // Two valid expressions to meet the constraint
    let result = compiler.c_alternate(&exprs);
    assert_eq!(result, Err(Error::Syntax("alternations cannot currently contain empty sub-expressions".to_string())));
}

