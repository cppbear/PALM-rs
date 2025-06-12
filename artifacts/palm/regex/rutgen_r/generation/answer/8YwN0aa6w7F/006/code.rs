// Answer 0

#[test]
fn test_hirkind_has_subexprs_empty() {
    struct HirKind {
        kind: String,
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match self.kind.as_str() {
                "Empty" | "Literal" | "Class" | "Anchor" | "WordBoundary" => false,
                "Group" | "Repetition" | "Concat" | "Alternation" => true,
                _ => unreachable!(),
            }
        }
    }

    let empty = HirKind { kind: String::from("Empty") };
    assert_eq!(empty.has_subexprs(), false);

    let literal = HirKind { kind: String::from("Literal") };
    assert_eq!(literal.has_subexprs(), false);

    let class = HirKind { kind: String::from("Class") };
    assert_eq!(class.has_subexprs(), false);

    let anchor = HirKind { kind: String::from("Anchor") };
    assert_eq!(anchor.has_subexprs(), false);

    let word_boundary = HirKind { kind: String::from("WordBoundary") };
    assert_eq!(word_boundary.has_subexprs(), false);
}

