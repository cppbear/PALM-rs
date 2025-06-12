// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii_case1() {
    let input_data = CharInput(b"hello");
    let at = InputAt { pos: 2, c: Char(101), byte: Some(b'e'), len: 5 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_case2() {
    let input_data = CharInput(b"12345");
    let at = InputAt { pos: 2, c: Char(50), byte: Some(b'3'), len: 5 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_case3() {
    let input_data = CharInput(b"abc def");
    let at = InputAt { pos: 3, c: Char(100), byte: Some(b' '), len: 7 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_case4() {
    let input_data = CharInput(b"Hello, World!");
    let at = InputAt { pos: 5, c: Char(111), byte: Some(b'o'), len: 13 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_case5() {
    let input_data = CharInput(b"word123");
    let at = InputAt { pos: 4, c: Char(49), byte: Some(b'1'), len: 8 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    input_data.is_empty_match(at, &empty);
}

