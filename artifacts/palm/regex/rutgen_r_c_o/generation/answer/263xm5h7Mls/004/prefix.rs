// Answer 0

#[test]
fn test_is_empty_match_word_boundary_true() {
    let input_data = b"a".to_vec();
    let input = CharInput(&input_data);
    let at = InputAt { pos: 1, c: Char(97), byte: Some(97), len: 1 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::WordBoundary };

    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_false() {
    let input_data = b"1".to_vec();
    let input = CharInput(&input_data);
    let at = InputAt { pos: 1, c: Char(49), byte: Some(49), len: 1 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::WordBoundary };

    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_edge_case() {
    let input_data = b" ".to_vec();
    let input = CharInput(&input_data);
    let at = InputAt { pos: 1, c: Char(32), byte: Some(32), len: 1 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::WordBoundary };

    input.is_empty_match(at, &empty);
}

