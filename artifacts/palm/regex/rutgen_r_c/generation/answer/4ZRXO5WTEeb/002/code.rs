// Answer 0

#[test]
fn test_is_match_true() {
    let instruction = Inst::Match(0);
    assert!(instruction.is_match());
}

#[test]
fn test_is_match_true_with_non_zero_index() {
    let instruction = Inst::Match(3);
    assert!(instruction.is_match());
}

#[test]
fn test_is_match_false_for_char() {
    let instruction = Inst::Char(InstChar { goto: 1, c: 'a' });
    assert!(!instruction.is_match());
}

#[test]
fn test_is_match_false_for_ranges() {
    let instruction = Inst::Ranges(InstRanges { goto: 1, ranges: vec![('a', 'z')] });
    assert!(!instruction.is_match());
}

#[test]
fn test_is_match_false_for_bytes() {
    let instruction = Inst::Bytes(InstBytes { goto: 1, start: 0, end: 255 });
    assert!(!instruction.is_match());
}

#[test]
fn test_is_match_false_for_save() {
    let instruction = Inst::Save(InstSave { goto: 1, slot: 0 });
    assert!(!instruction.is_match());
}

#[test]
fn test_is_match_false_for_split() {
    let instruction = Inst::Split(InstSplit { goto1: 1, goto2: 2 });
    assert!(!instruction.is_match());
}

#[test]
fn test_is_match_false_for_empty_look() {
    let instruction = Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::ZeroWidth });
    assert!(!instruction.is_match());
}

