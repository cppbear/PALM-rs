// Answer 0

#[test]
fn test_inst_is_match_with_match_variant() {
    let inst = Inst::Match(0);
    assert!(inst.is_match());
}

#[test]
fn test_inst_is_match_with_save_variant() {
    let inst = Inst::Save(InstSave { goto: 1, slot: 0 });
    assert!(!inst.is_match());
}

#[test]
fn test_inst_is_match_with_split_variant() {
    let inst = Inst::Split(InstSplit { goto1: 1, goto2: 2 });
    assert!(!inst.is_match());
}

#[test]
fn test_inst_is_match_with_empty_look_variant() {
    let inst = Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::ZeroWidth });
    assert!(!inst.is_match());
}

#[test]
fn test_inst_is_match_with_char_variant() {
    let inst = Inst::Char(InstChar { goto: 1, c: 'a' });
    assert!(!inst.is_match());
}

#[test]
fn test_inst_is_match_with_ranges_variant() {
    let inst = Inst::Ranges(InstRanges { goto: 1, ranges: vec![('a', 'z')] });
    assert!(!inst.is_match());
}

#[test]
fn test_inst_is_match_with_bytes_variant() {
    let inst = Inst::Bytes(InstBytes { goto: 1, start: 0, end: 255 });
    assert!(!inst.is_match());
}

#[test]
fn test_inst_is_match_with_multiple_variants() {
    let inst_match = Inst::Match(1);
    let inst_char = Inst::Char(InstChar { goto: 2, c: 'b' });

    assert!(inst_match.is_match());
    assert!(!inst_char.is_match());
}

