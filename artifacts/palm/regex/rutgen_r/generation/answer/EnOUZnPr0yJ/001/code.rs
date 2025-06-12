// Answer 0

#[test]
#[should_panic]
fn test_c_class_with_empty_ranges() {
    struct CompileContext {
        compiled: Compiled,
        insts: Vec<Inst>,
    }

    struct Compiled {
        uses_bytes: bool,
    }

    struct Inst;

    impl CompileContext {
        fn c_class(&mut self, ranges: &[hir::ClassUnicodeRange]) -> Result {
            assert!(!ranges.is_empty());
            if self.compiled.uses_bytes {
                CompileClass {
                    c: self,
                    ranges: ranges,
                }.compile()
            } else {
                let ranges: Vec<(char, char)> = 
                    ranges.iter().map(|r| (r.start(), r.end())).collect();
                let hole = if ranges.len() == 1 && ranges[0].0 == ranges[0].1 {
                    self.push_hole(InstHole::Char { c: ranges[0].0 })
                } else {
                    self.push_hole(InstHole::Ranges { ranges: ranges })
                };
                Ok(Patch { hole: hole, entry: self.insts.len() - 1 })
            }
        }

        fn push_hole(&mut self, _hole: InstHole) -> Hole {
            Hole
        }
    }

    struct CompileClass<'a> {
        c: &'a mut CompileContext,
        ranges: &'a [hir::ClassUnicodeRange],
    }

    impl<'a> CompileClass<'a> {
        fn compile(self) -> Result {
            Ok(())
        }
    }

    struct Patch {
        hole: Hole,
        entry: usize,
    }

    struct Hole;

    struct Result;

    let mut context = CompileContext {
        compiled: Compiled { uses_bytes: false },
        insts: Vec::new(),
    };

    context.c_class(&[]);
}

