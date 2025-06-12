// Answer 0

#[derive(Clone, Debug)]
struct Literal {
    value: String,
    cut: bool,
}

impl Literal {
    fn new(value: &str) -> Self {
        Literal {
            value: value.to_string(),
            cut: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn clear(&mut self) {
        self.value.clear();
    }

    fn cut(&mut self) {
        self.cut = true;
    }
}

#[derive(Debug)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn new() -> Self {
        Literals { lits: Vec::new() }
    }

    fn to_empty(&self) -> Self {
        Literals::new()
    }
}

#[test]
fn test_unambiguous_prefixes_with_various_literals() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("abc"));
    literals.lits.push(Literal::new("ab"));
    literals.lits.push(Literal::new("abcdef"));
    literals.lits.push(Literal::new("a"));

    let prefixes = literals.unambiguous_prefixes();
    assert_eq!(prefixes.lits.len(), 2);
    assert_eq!(prefixes.lits[0].value, "abc");
    assert_eq!(prefixes.lits[1].value, "a");
}

#[test]
fn test_unambiguous_prefixes_no_empty_literals() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("x"));
    literals.lits.push(Literal::new("y"));
    literals.lits.push(Literal::new("xy"));

    let prefixes = literals.unambiguous_prefixes();
    assert_eq!(prefixes.lits.len(), 2);
    assert_eq!(prefixes.lits[0].value, "x");
    assert_eq!(prefixes.lits[1].value, "y");
}

#[test]
fn test_unambiguous_prefixes_with_identical_prefixes() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("hello"));
    literals.lits.push(Literal::new("hell"));
    literals.lits.push(Literal::new("he"));

    let prefixes = literals.unambiguous_prefixes();
    assert_eq!(prefixes.lits.len(), 1);
    assert_eq!(prefixes.lits[0].value, "hello");
}

#[test]
fn test_unambiguous_prefixes_with_empty_set() {
    let literals = Literals::new();

    let prefixes = literals.unambiguous_prefixes();
    assert_eq!(prefixes.lits.len(), 0);
}

#[test]
fn test_unambiguous_prefixes_with_cut_literals() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("foo"));
    literals.lits.push(Literal::new("fo"));
    literals.lits.push(Literal::new("foobar"));

    let prefixes = literals.unambiguous_prefixes();
    assert_eq!(prefixes.lits.len(), 2);
    assert_eq!(prefixes.lits[0].value, "foo");
    assert_eq!(prefixes.lits[1].value, "fo");
}

