// Answer 0

fn test_c_class_single_range() {
    struct CompileContext {
        compiled: Compiled,
        insts: Vec<Inst>,
    }

    struct Compiled {
        use_bytes: bool,
    }

    struct Inst;

    struct Patch {
        hole: InstHole,
        entry: usize,
    }

    enum InstHole {
        Char { c: char },
        Ranges { ranges: Vec<(char, char)> },
    }

    impl CompileContext {
        fn push_hole(&mut self, hole: InstHole) -> InstHole {
            self.insts.push(Inst);
            hole
        }

        fn c_class(&mut self, ranges: &[ClassUnicodeRange]) -> Result<Patch, String> {
            assert!(!ranges.is_empty());
            if self.compiled.uses_bytes {
                Err("uses_bytes is true".to_string())
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
    }

    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    impl ClassUnicodeRange {
        fn start(&self) -> char {
            self.start
        }

        fn end(&self) -> char {
            self.end
        }
    }

    let mut context = CompileContext {
        compiled: Compiled { use_bytes: false },
        insts: Vec::new(),
    };

    let ranges = vec![ClassUnicodeRange { start: 'a', end: 'a' }];

    match context.c_class(&ranges) {
        Ok(patch) => {
            assert_eq!(patch.entry, 0);
            if let InstHole::Char { c } = patch.hole {
                assert_eq!(c, 'a');
            } else {
                panic!("Expected hole to be of type Char with value 'a'");
            }
        }
        Err(_) => panic!("Function should not return an error"),
    }
}

