// Answer 0

#[test]
#[should_panic(expected = "alternates must have at least 2 exprs")]
fn test_c_alternate_with_one_expression() {
    struct Hir;
    struct Hole;
    struct Patch {
        hole: Hole,
        entry: usize,
    }
    struct Compiler {
        insts: Vec<usize>,
    }

    impl Compiler {
        fn new() -> Self {
            Self { insts: vec![] }
        }

        fn c(&mut self, _expr: &Hir) -> Result<Patch, Error> {
            // Simulating the compiler's behavior
            Ok(Patch { hole: Hole, entry: self.insts.len() })
        }

        fn fill_to_next(&mut self, _hole: Hole) {}

        fn push_split_hole(&mut self) -> Hole {
            Hole
        }

        fn fill_split(&mut self, _split: Hole, _entry: Option<usize>, _arg: Option<usize>) -> Hole {
            Hole
        }

        fn fill(&mut self, _prev_hole: Hole, _entry: usize) {}

        fn c_alternate(&mut self, exprs: &[Hir]) -> Result<Patch, Error> {
            // The original function to be tested
            debug_assert!(
                exprs.len() >= 2, "alternates must have at least 2 exprs");

            let first_split_entry = self.insts.len();
            let mut holes = vec![];
            let mut prev_hole = Hole;

            for e in &exprs[0..exprs.len() - 1] {
                self.fill_to_next(prev_hole);
                let split = self.push_split_hole();
                let prev_entry = self.insts.len();
                let patch = self.c(e)?;
                holes.push(patch.hole);
                prev_hole = self.fill_split(split, Some(patch.entry), None);
            }
            let prev_entry = self.insts.len();
            let patch = self.c(&exprs[exprs.len() - 1])?;
            holes.push(patch.hole);
            self.fill(prev_hole, patch.entry);
            Ok(Patch { hole: Hole::Many(holes), entry: first_split_entry })
        }
    }

    struct Error;

    type Result<T, E> = std::result::Result<T, E>;

    let mut compiler = Compiler::new();
    let exprs = vec![Hir]; // Only one expression, triggering the panic
    compiler.c_alternate(&exprs).unwrap();
}

