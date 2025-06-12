// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_case1() {
    let input_data = b"Hello, world!";
    let char_input = CharInput(input_data);
    let inst_empty_look = InstEmptyLook { goto: InstPtr::default(), look: NotWordBoundary };
    let input_at = InputAt { pos: 5, c: Char(108), byte: Some(b'l'), len: 13 };

    let _ = char_input.is_empty_match(input_at, &inst_empty_look);
}

#[test]
fn test_is_empty_match_not_word_boundary_case2() {
    let input_data = b"1234!5678";
    let char_input = CharInput(input_data);
    let inst_empty_look = InstEmptyLook { goto: InstPtr::default(), look: NotWordBoundary };
    let input_at = InputAt { pos: 4, c: Char(33), byte: Some(b'!'), len: 10 };

    let _ = char_input.is_empty_match(input_at, &inst_empty_look);
}

#[test]
fn test_is_empty_match_not_word_boundary_case3() {
    let input_data = b"word word!";
    let char_input = CharInput(input_data);
    let inst_empty_look = InstEmptyLook { goto: InstPtr::default(), look: NotWordBoundary };
    let input_at = InputAt { pos: 4, c: Char(32), byte: Some(b' '), len: 10 };

    let _ = char_input.is_empty_match(input_at, &inst_empty_look);
}

#[test]
fn test_is_empty_match_not_word_boundary_case4() {
    let input_data = b"   ";
    let char_input = CharInput(input_data);
    let inst_empty_look = InstEmptyLook { goto: InstPtr::default(), look: NotWordBoundary };
    let input_at = InputAt { pos: 1, c: Char(32), byte: Some(b' '), len: 3 };

    let _ = char_input.is_empty_match(input_at, &inst_empty_look);
}

#[test]
fn test_is_empty_match_not_word_boundary_case5() {
    let input_data = b"!@#$%^&*()";
    let char_input = CharInput(input_data);
    let inst_empty_look = InstEmptyLook { goto: InstPtr::default(), look: NotWordBoundary };
    let input_at = InputAt { pos: 3, c: Char(36), byte: Some(b'$'), len: 10 };

    let _ = char_input.is_empty_match(input_at, &inst_empty_look);
}

