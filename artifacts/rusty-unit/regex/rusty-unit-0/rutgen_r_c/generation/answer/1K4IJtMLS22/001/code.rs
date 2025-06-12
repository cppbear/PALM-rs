// Answer 0

#[test]
fn test_push_hole_empty_insts() {
    let mut compiler = Compiler::new();
    let inst_hole = InstHole::Char { c: 'a' };
    
    let result = compiler.push_hole(inst_hole);
    
    assert_eq!(result, Hole::One(0));
}

#[test]
fn test_push_hole_non_empty_insts() {
    let mut compiler = Compiler::new();
    let inst_hole1 = InstHole::Char { c: 'a' };
    let inst_hole2 = InstHole::Char { c: 'b' };
    
    // Push the first hole
    compiler.push_hole(inst_hole1);
    
    // Push the second hole
    let result = compiler.push_hole(inst_hole2);
    
    assert_eq!(result, Hole::One(1));
}

#[test]
fn test_push_hole_multiple_holes() {
    let mut compiler = Compiler::new();
    let inst_hole1 = InstHole::EmptyLook { look: EmptyLook {} };
    let inst_hole2 = InstHole::Ranges { ranges: vec![('a', 'z')] };
    
    // Push first hole
    let result1 = compiler.push_hole(inst_hole1);
    assert_eq!(result1, Hole::One(0));
    
    // Push second hole
    let result2 = compiler.push_hole(inst_hole2);
    assert_eq!(result2, Hole::One(1));
}

