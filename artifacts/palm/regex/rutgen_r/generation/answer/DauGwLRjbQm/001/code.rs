// Answer 0

#[test]
#[should_panic(expected = "non-empty literal")]
fn test_c_literal_empty_chars() {
    struct DummyCompiled {
        is_reverse: bool,
    }

    struct Dummy {
        compiled: DummyCompiled,
    }

    impl Dummy {
        fn c_char(&self, _c: char) -> Result {
            // Example implementation that doesn't panic, returning a dummy Patch
            Ok(Patch { hole: 0, entry: 1 })
        }

        fn fill(&self, _hole: usize, _entry: usize) {
            // Dummy fill implementation; does nothing
        }

        fn c_literal(&mut self, chars: &[char]) -> Result {
            debug_assert!(!chars.is_empty());
            let mut chars: Box<Iterator<Item=&char>> =
                if self.compiled.is_reverse {
                    Box::new(chars.iter().rev())
                } else {
                    Box::new(chars.iter())
                };
            let first = *chars.next().expect("non-empty literal");
            let Patch { mut hole, entry } = self.c_char(first)?;
            for &c in chars {
                let p = self.c_char(c)?;
                self.fill(hole, p.entry);
                hole = p.hole;
            }
            Ok(Patch { hole: hole, entry: entry })
        }
    }

    struct Patch {
        hole: usize,
        entry: usize,
    }

    let mut dummy = Dummy {
        compiled: DummyCompiled { is_reverse: false },
    };

    dummy.c_literal(&[]);
}

