// Answer 0

#[test]
fn test_fill_inst_hole_empty_look() {
    let goto = InstPtr(1);
    let hole = InstHole::EmptyLook { look: EmptyLook::StartLine };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::StartLine);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_end_line() {
    let goto = InstPtr(2);
    let hole = InstHole::EmptyLook { look: EmptyLook::EndLine };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::EndLine);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_start_text() {
    let goto = InstPtr(3);
    let hole = InstHole::EmptyLook { look: EmptyLook::StartText };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::StartText);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_end_text() {
    let goto = InstPtr(4);
    let hole = InstHole::EmptyLook { look: EmptyLook::EndText };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::EndText);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_word_boundary() {
    let goto = InstPtr(5);
    let hole = InstHole::EmptyLook { look: EmptyLook::WordBoundary };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::WordBoundary);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_not_word_boundary() {
    let goto = InstPtr(6);
    let hole = InstHole::EmptyLook { look: EmptyLook::NotWordBoundary };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::NotWordBoundary);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_word_boundary_ascii() {
    let goto = InstPtr(7);
    let hole = InstHole::EmptyLook { look: EmptyLook::WordBoundaryAscii };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::WordBoundaryAscii);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

#[test]
fn test_fill_inst_hole_empty_look_not_word_boundary_ascii() {
    let goto = InstPtr(8);
    let hole = InstHole::EmptyLook { look: EmptyLook::NotWordBoundaryAscii };
    
    let result = hole.fill(goto);
    
    if let Inst::EmptyLook(inst_empty_look) = result {
        assert_eq!(inst_empty_look.goto, goto);
        assert_eq!(inst_empty_look.look, EmptyLook::NotWordBoundaryAscii);
    } else {
        panic!("Expected Inst::EmptyLook variant");
    }
}

