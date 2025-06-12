// Answer 0

#[derive(Debug)]
struct Literals {
    lits: Vec<String>,
    limit_size: usize,
    limit_class: char,
}

impl Literals {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Literals")
            .field("lits", &self.lits)
            .field("limit_size", &self.limit_size)
            .field("limit_class", &self.limit_class)
            .finish()
    }
}

#[test]
fn test_fmt_empty_literals() {
    let literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 'a',
    };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        literals.fmt(formatter).unwrap();
    }
    assert!(output.contains("lits: []"));
    assert!(output.contains("limit_size: 0"));
    assert!(output.contains("limit_class: 'a'"));
}

#[test]
fn test_fmt_non_empty_literals() {
    let literals = Literals {
        lits: vec!["abc".to_string(), "def".to_string()],
        limit_size: 5,
        limit_class: 'b',
    };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        literals.fmt(formatter).unwrap();
    }
    assert!(output.contains("lits: [\"abc\", \"def\"]"));
    assert!(output.contains("limit_size: 5"));
    assert!(output.contains("limit_class: 'b'"));
}

