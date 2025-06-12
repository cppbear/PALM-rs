// Answer 0

#[test]
fn test_is_match_with_save() {
    let instruction = Inst::Save(InstSave { goto: 0, slot: 0 });
    instruction.is_match();
}

#[test]
fn test_is_match_with_split() {
    let instruction = Inst::Split(InstSplit { goto1: 0, goto2: 1 });
    instruction.is_match();
}

#[test]
fn test_is_match_with_empty_look() {
    let instruction = Inst::EmptyLook(InstEmptyLook { goto: 0, look: EmptyLook::ZeroWidth });
    instruction.is_match();
}

#[test]
fn test_is_match_with_char() {
    let instruction = Inst::Char(InstChar { goto: 0, c: 'a' });
    instruction.is_match();
}

#[test]
fn test_is_match_with_ranges() {
    let instruction = Inst::Ranges(InstRanges { goto: 0, ranges: vec![('a', 'z')] });
    instruction.is_match();
}

#[test]
fn test_is_match_with_bytes() {
    let instruction = Inst::Bytes(InstBytes { goto: 0, start: 0, end: 255 });
    instruction.is_match();
}

