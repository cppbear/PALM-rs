// Answer 0

#[derive(Debug)]
struct Patch {
    hole: usize,
    entry: usize,
}

struct Compiler {
    compiled: Compiled,
}

struct Compiled {
    is_reverse: bool,
}

impl Compiler {
    fn c_char(&mut self, c: char) -> Result<Patch> {
        // Dummy implementation for testing purposes
        Ok(Patch { hole: c as usize, entry: c as usize })
    }

    fn fill(&mut self, hole: usize, entry: usize) {
        // Dummy implementation
    }

    fn c_literal(&mut self, chars: &[char]) -> Result<Patch> {
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

type Result<T> = std::result::Result<T, &'static str>;

#[test]
fn test_c_literal() {
    let mut compiler = Compiler {
        compiled: Compiled { is_reverse: false },
    };
    let result = compiler.c_literal(&['a', 'b', 'c']);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.hole, 'c' as usize);
    assert_eq!(patch.entry, 'a' as usize);
}

#[test]
fn test_c_literal_reverse() {
    let mut compiler = Compiler {
        compiled: Compiled { is_reverse: true },
    };
    let result = compiler.c_literal(&['a', 'b', 'c']);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.hole, 'a' as usize);
    assert_eq!(patch.entry, 'c' as usize);
}

#[test]
#[should_panic(expected = "non-empty literal")]
fn test_c_literal_empty() {
    let mut compiler = Compiler {
        compiled: Compiled { is_reverse: false },
    };
    compiler.c_literal(&[]);
}

