// Answer 0

#[test]
fn test_is_empty_match_start_line_not_start() {
    let input_data = b"abc";
    let input = CharInput(input_data);
    let at = InputAt { pos: 1, c: Char(98), byte: Some(b'b'), len: input.len() };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::StartLine };
    
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_start_line_char_not_newline() {
    let input_data = b"\nabc";
    let input = CharInput(input_data);
    let at = InputAt { pos: 1, c: Char(98), byte: Some(b'b'), len: input.len() };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::StartLine };

    input.is_empty_match(at, &empty);
}

