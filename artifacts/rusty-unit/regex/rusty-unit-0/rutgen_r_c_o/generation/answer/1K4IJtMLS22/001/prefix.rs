// Answer 0

#[test]
fn test_push_hole_empty() {
    let mut compiler = Compiler::new();
    let inst = InstHole::Char { c: 'a' };
    compiler.push_hole(inst);
}

#[test]
fn test_push_hole_single_entry() {
    let mut compiler = Compiler::new();
    let inst = InstHole::Save { slot: 0 };
    compiler.push_hole(inst);
}

#[test]
fn test_push_hole_multiple_entries() {
    let mut compiler = Compiler::new();
    for i in 0..10 {
        let inst = InstHole::Bytes { start: i as u8, end: (i + 1) as u8 };
        compiler.push_hole(inst);
    }
}

#[test]
fn test_push_hole_with_empty_look() {
    let mut compiler = Compiler::new();
    let inst = InstHole::EmptyLook { look: EmptyLook::Zero };
    compiler.push_hole(inst);
}

#[test]
fn test_push_hole_at_capacity() {
    let mut compiler = Compiler::new();
    for i in 0..255 {
        let inst = InstHole::Ranges { ranges: vec![(i as char, (i + 1) as char)] };
        compiler.push_hole(inst);
    }
    
    let inst = InstHole::Char { c: 'z' };
    compiler.push_hole(inst);
}

