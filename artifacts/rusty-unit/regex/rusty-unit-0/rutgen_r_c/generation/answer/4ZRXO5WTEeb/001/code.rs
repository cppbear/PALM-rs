// Answer 0

#[test]
fn test_is_match_with_char_instruction() {
    let instruction = Inst::Char(InstChar { goto: 1, c: 'a' });
    assert_eq!(instruction.is_match(), false);
}

#[test]
fn test_is_match_with_ranges_instruction() {
    let instruction = Inst::Ranges(InstRanges { goto: 1, ranges: vec![('a', 'z')] });
    assert_eq!(instruction.is_match(), false);
}

#[test]
fn test_is_match_with_bytes_instruction() {
    let instruction = Inst::Bytes(InstBytes { goto: 1, start: 1, end: 255 });
    assert_eq!(instruction.is_match(), false);
}

#[test]
fn test_is_match_with_save_instruction() {
    let instruction = Inst::Save(InstSave { goto: 1, slot: 0 });
    assert_eq!(instruction.is_match(), false);
}

#[test]
fn test_is_match_with_split_instruction() {
    let instruction = Inst::Split(InstSplit { goto1: 1, goto2: 2 });
    assert_eq!(instruction.is_match(), false);
}

#[test]
fn test_is_match_with_empty_look_instruction() {
    let instruction = Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::ZeroWidth });
    assert_eq!(instruction.is_match(), false);
}

