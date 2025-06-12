// Answer 0

#[derive(Debug)]
struct Literals {
    lits: Vec<String>,
    limit_size: usize,
    limit_class: String,
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
fn test_fmt_empty_lits() {
    let literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: String::from(""),
    };
    let mut output = std::fmt::Formatter::new();
    assert!(literals.fmt(&mut output).is_ok());
}

#[test]
fn test_fmt_single_literal() {
    let literals = Literals {
        lits: vec![String::from("a")],
        limit_size: 1,
        limit_class: String::from("class_a"),
    };
    let mut output = std::fmt::Formatter::new();
    assert!(literals.fmt(&mut output).is_ok());
}

#[test]
fn test_fmt_multiple_literals() {
    let literals = Literals {
        lits: vec![String::from("a"), String::from("b"), String::from("c")],
        limit_size: 3,
        limit_class: String::from("class_b"),
    };
    let mut output = std::fmt::Formatter::new();
    assert!(literals.fmt(&mut output).is_ok());
}

#[test]
fn test_fmt_large_limit_size() {
    let literals = Literals {
        lits: vec![String::from("large_limit")],
        limit_size: usize::MAX,
        limit_class: String::from("class_large"),
    };
    let mut output = std::fmt::Formatter::new();
    assert!(literals.fmt(&mut output).is_ok());
}

#[test]
fn test_fmt_special_character_limit_class() {
    let literals = Literals {
        lits: vec![String::from("special_char")],
        limit_size: 1,
        limit_class: String::from("class@&*"),
    };
    let mut output = std::fmt::Formatter::new();
    assert!(literals.fmt(&mut output).is_ok());
}

