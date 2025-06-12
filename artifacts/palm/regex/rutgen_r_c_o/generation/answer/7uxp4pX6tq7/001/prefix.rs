// Answer 0

#[test]
fn test_fill_with_compiled_instruction() {
    let mut inst = MaybeInst::Compiled(Inst::Match(0));
    let goto: InstPtr = 1;
    inst.fill(goto);
}

#[test]
fn test_fill_with_split_instruction() {
    let mut inst = MaybeInst::Split;
    let goto: InstPtr = 2;
    inst.fill(goto);
}

#[test]
fn test_fill_with_empty_look_instruction() {
    let mut inst = MaybeInst::EmptyLook(InstEmptyLook { goto: 3, look: EmptyLook });
    let goto: InstPtr = 4;
    inst.fill(goto);
}

#[test]
fn test_fill_with_char_instruction() {
    let mut inst = MaybeInst::Char(InstChar { goto: 5, c: 'a' });
    let goto: InstPtr = 6;
    inst.fill(goto);
}

#[test]
fn test_fill_with_ranges_instruction() {
    let ranges = vec![('a', 'z'), ('A', 'Z')];
    let mut inst = MaybeInst::Ranges(InstRanges { goto: 7, ranges });
    let goto: InstPtr = 8;
    inst.fill(goto);
}

#[test]
fn test_fill_with_bytes_instruction() {
    let mut inst = MaybeInst::Bytes(InstBytes { goto: 9, start: 10, end: 255 });
    let goto: InstPtr = 11;
    inst.fill(goto);
}

