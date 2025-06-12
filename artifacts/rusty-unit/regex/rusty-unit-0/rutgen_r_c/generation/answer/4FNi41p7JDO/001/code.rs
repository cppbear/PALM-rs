// Answer 0

#[test]
fn test_fmt_empty_hir() {
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo { bools: 0 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_literal_hir() {
    let literal_character = 'a';
    let hir = Hir {
        kind: HirKind::Literal(Literal::from(literal_character)),
        info: HirInfo { bools: 0b00000001 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_class_hir() {
    let class = Class::new(vec!['a', 'b', 'c']);
    let hir = Hir {
        kind: HirKind::Class(class),
        info: HirInfo { bools: 0b00000010 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_anchor_hir() {
    let anchor = Anchor::start_of_string();
    let hir = Hir {
        kind: HirKind::Anchor(anchor),
        info: HirInfo { bools: 0b00000100 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_word_boundary_hir() {
    let word_boundary = WordBoundary::new();
    let hir = Hir {
        kind: HirKind::WordBoundary(word_boundary),
        info: HirInfo { bools: 0b00001000 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_repetition_hir() {
    let repetition = Repetition::new();
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        info: HirInfo { bools: 0b00010000 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_group_hir() {
    let group = Group::new(vec![]);
    let hir = Hir {
        kind: HirKind::Group(group),
        info: HirInfo { bools: 0b00100000 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_concat_hir() {
    let concat = Vec::new(); // Replace with actual Hir instances
    let hir = Hir {
        kind: HirKind::Concat(concat),
        info: HirInfo { bools: 0b01000000 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

#[test]
fn test_fmt_alternation_hir() {
    let alternation = Vec::new(); // Replace with actual Hir instances
    let hir = Hir {
        kind: HirKind::Alternation(alternation),
        info: HirInfo { bools: 0b10000000 },
    };
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", hir));
    assert!(result.is_ok());
}

