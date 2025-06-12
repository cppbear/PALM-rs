// Answer 0

#[test]
fn test_is_empty_match_end_line() {
    struct TestInput<'t> {
        data: ByteInput<'t>,
        position: InputAt,
        empty: InstEmptyLook,
    }

    let text: &[u8] = b"Hello\nWorld";
    let input = ByteInput { text, only_utf8: true };

    let empty_look = InstEmptyLook {
        goto: InstPtr::new(), // Assume InstPtr::new() is a valid constructor.
        look: prog::EmptyLook::EndLine,
    };

    let position = InputAt {
        pos: 10, // This is the end of the string
        c: Char(u32::MAX), // Use a Char value that represents none.
        byte: None,
        len: 0,
    };

    let test_input = TestInput { data: input, position, empty: empty_look };

    assert!(test_input.data.is_empty_match(test_input.position, &test_input.empty));
}

#[test]
fn test_is_empty_match_not_end_line() {
    struct TestInput<'t> {
        data: ByteInput<'t>,
        position: InputAt,
        empty: InstEmptyLook,
    }

    let text: &[u8] = b"Hello\nWorld";
    let input = ByteInput { text, only_utf8: true };

    let empty_look = InstEmptyLook {
        goto: InstPtr::new(),
        look: prog::EmptyLook::EndLine,
    };

    let position = InputAt {
        pos: 0, // This is not at the end
        c: Char(0),
        byte: None,
        len: 0,
    };

    let test_input = TestInput { data: input, position, empty: empty_look };

    assert!(!test_input.data.is_empty_match(test_input.position, &test_input.empty));
}

