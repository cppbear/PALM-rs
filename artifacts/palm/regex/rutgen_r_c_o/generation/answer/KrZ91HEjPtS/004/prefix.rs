// Answer 0

#[test]
fn test_fill_empty_look_start_line() {
    let hole = InstHole::EmptyLook { look: EmptyLook::StartLine };
    let goto = 1;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_end_line() {
    let hole = InstHole::EmptyLook { look: EmptyLook::EndLine };
    let goto = 2;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_start_text() {
    let hole = InstHole::EmptyLook { look: EmptyLook::StartText };
    let goto = 3;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_end_text() {
    let hole = InstHole::EmptyLook { look: EmptyLook::EndText };
    let goto = 4;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_word_boundary() {
    let hole = InstHole::EmptyLook { look: EmptyLook::WordBoundary };
    let goto = 5;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_not_word_boundary() {
    let hole = InstHole::EmptyLook { look: EmptyLook::NotWordBoundary };
    let goto = 6;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_word_boundary_ascii() {
    let hole = InstHole::EmptyLook { look: EmptyLook::WordBoundaryAscii };
    let goto = 7;
    hole.fill(goto);
}

#[test]
fn test_fill_empty_look_not_word_boundary_ascii() {
    let hole = InstHole::EmptyLook { look: EmptyLook::NotWordBoundaryAscii };
    let goto = 8;
    hole.fill(goto);
}

