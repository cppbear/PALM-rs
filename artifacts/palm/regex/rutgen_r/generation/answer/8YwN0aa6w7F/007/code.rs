// Answer 0

#[test]
fn test_has_subexprs_class() {
    struct DummyClass;
    
    enum HirKind {
        Empty,
        Literal(char),
        Class(DummyClass),
        Anchor(char),
        WordBoundary(char),
        Group(Box<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Box<HirKind>, Box<HirKind>),
        Alternation(Box<HirKind>, Box<HirKind>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match *self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_) 
                | HirKind::Repetition(_) 
                | HirKind::Concat(_, _) 
                | HirKind::Alternation(_, _) => true,
            }
        }
    }

    let class_instance = HirKind::Class(DummyClass);
    assert_eq!(class_instance.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_word_boundary() {
    struct DummyWordBoundary;
    
    enum HirKind {
        Empty,
        Literal(char),
        Class(DummyWordBoundary),
        Anchor(char),
        WordBoundary(char),
        Group(Box<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Box<HirKind>, Box<HirKind>),
        Alternation(Box<HirKind>, Box<HirKind>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match *self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_) 
                | HirKind::Repetition(_) 
                | HirKind::Concat(_, _) 
                | HirKind::Alternation(_, _) => true,
            }
        }
    }

    let word_boundary_instance = HirKind::WordBoundary('w');
    assert_eq!(word_boundary_instance.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_anchor() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(String),
        Anchor(char),
        WordBoundary(char),
        Group(Box<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Box<HirKind>, Box<HirKind>),
        Alternation(Box<HirKind>, Box<HirKind>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match *self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_) 
                | HirKind::Repetition(_) 
                | HirKind::Concat(_, _) 
                | HirKind::Alternation(_, _) => true,
            }
        }
    }

    let anchor_instance = HirKind::Anchor('^');
    assert_eq!(anchor_instance.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_empty() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(String),
        Anchor(char),
        WordBoundary(char),
        Group(Box<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Box<HirKind>, Box<HirKind>),
        Alternation(Box<HirKind>, Box<HirKind>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match *self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_) 
                | HirKind::Repetition(_) 
                | HirKind::Concat(_, _) 
                | HirKind::Alternation(_, _) => true,
            }
        }
    }

    let empty_instance = HirKind::Empty;
    assert_eq!(empty_instance.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    enum HirKind {
        Empty,
        Literal(char),
        Class(String),
        Anchor(char),
        WordBoundary(char),
        Group(Box<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Box<HirKind>, Box<HirKind>),
        Alternation(Box<HirKind>, Box<HirKind>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match *self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_) 
                | HirKind::Repetition(_) 
                | HirKind::Concat(_, _) 
                | HirKind::Alternation(_, _) => true,
            }
        }
    }

    let literal_instance = HirKind::Literal('a');
    assert_eq!(literal_instance.has_subexprs(), false);
}

