// Answer 0

#[test]
fn test_is_empty_match_end_text_at_start() {
    let input_data = b"";
    let input = CharInput(input_data);
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::EndText };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    let _ = input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_text_at_end() {
    let input_data = b"Hello";
    let input = CharInput(input_data);
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::EndText };
    let at = InputAt { pos: 5, c: Char(0), byte: None, len: 5 };
    let _ = input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_text_mid_position() {
    let input_data = b"Hello";
    let input = CharInput(input_data);
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::EndText };
    let at = InputAt { pos: 3, c: Char(0), byte: None, len: 5 };
    let _ = input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_text_on_single_char() {
    let input_data = b"A";
    let input = CharInput(input_data);
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::EndText };
    let at = InputAt { pos: 1, c: Char(65), byte: Some(65), len: 1 };
    let _ = input.is_empty_match(at, &empty);
}

