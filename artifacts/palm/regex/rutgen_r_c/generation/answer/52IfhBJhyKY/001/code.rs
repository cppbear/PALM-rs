// Answer 0

#[test]
fn test_push_compiled_with_match_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::Match(0);
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_with_save_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::Save(InstSave(1));
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_with_split_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::Split(InstSplit::new(0, 1));
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_with_empty_look_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::EmptyLook(InstEmptyLook);
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_with_char_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::Char(InstChar('a'));
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_with_ranges_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::Ranges(InstRanges::new(vec![(b'a', b'z')]));
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_with_bytes_inst() {
    let mut compiler = Compiler::new();
    let inst = Inst::Bytes(InstBytes::new(vec![b'a', b'b']));
    compiler.push_compiled(inst);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_multiple_insts() {
    let mut compiler = Compiler::new();
    let inst1 = Inst::Match(0);
    let inst2 = Inst::Char(InstChar('b'));
    let inst3 = Inst::Split(InstSplit::new(1, 2));
    
    compiler.push_compiled(inst1);
    compiler.push_compiled(inst2);
    compiler.push_compiled(inst3);
    
    assert_eq!(compiler.insts.len(), 3);
}

