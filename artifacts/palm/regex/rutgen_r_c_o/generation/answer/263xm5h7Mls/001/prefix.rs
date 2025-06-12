// Answer 0

#[test]
fn test_is_empty_match_start_line() {
    let input_data = CharInput(&b"example"[..]);
    let at = InputAt { pos: 0, c: Char(101), byte: None, len: 7 }; // 'e'
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::StartLine };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_line() {
    let input_data = CharInput(&b"example"[..]);
    let at = InputAt { pos: 7, c: Char(0), byte: None, len: 7 }; // end of string
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::EndLine };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_start_text() {
    let input_data = CharInput(&b"example"[..]);
    let at = InputAt { pos: 0, c: Char(101), byte: None, len: 7 }; // 'e'
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::StartText };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_text() {
    let input_data = CharInput(&b"example"[..]);
    let at = InputAt { pos: 7, c: Char(0), byte: None, len: 7 }; // end of string
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::EndText };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary() {
    let input_data = CharInput(&b"word word"[..]);
    let at = InputAt { pos: 4, c: Char(0), byte: None, len: 9 }; // position between "word" and "word"
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary() {
    let input_data = CharInput(&b"word word"[..]);
    let at = InputAt { pos: 4, c: Char(0), byte: None, len: 9 }; // position between "word" and "word"
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::NotWordBoundary };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii() {
    let input_data = CharInput(&b"word 1"[..]);
    let at = InputAt { pos: 4, c: Char(0), byte: None, len: 7 }; // position between "word" and "1"
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    let input_data = CharInput(&b"word 1"[..]);
    let at = InputAt { pos: 4, c: Char(0), byte: None, len: 7 }; // position between "word" and "1"
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::NotWordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

