// Answer 0

#[test]
fn test_is_empty_match_start_line() {
    use prog::EmptyLook::*;
    let input_bytes: &[u8] = b"\nHello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 0 };
    let empty_instruction = InstEmptyLook { goto: InstPtr(0), look: StartLine };

    assert!(input.is_empty_match(at, &empty_instruction));
}

#[test]
fn test_is_empty_match_end_line() {
    use prog::EmptyLook::*;
    let input_bytes: &[u8] = b"Hello, World!\n";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: input.len(), c: Char(u32::MAX), byte: None, len: 0 };
    let empty_instruction = InstEmptyLook { goto: InstPtr(0), look: EndLine };

    assert!(input.is_empty_match(at, &empty_instruction));
}

#[test]
fn test_is_empty_match_start_text() {
    use prog::EmptyLook::*;
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 0 };
    let empty_instruction = InstEmptyLook { goto: InstPtr(0), look: StartText };

    assert!(input.is_empty_match(at, &empty_instruction));
}

#[test]
fn test_is_empty_match_end_text() {
    use prog::EmptyLook::*;
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: input.len(), c: Char(u32::MAX), byte: None, len: 0 };
    let empty_instruction = InstEmptyLook { goto: InstPtr(0), look: EndText };

    assert!(input.is_empty_match(at, &empty_instruction));
}

#[test]
fn test_is_empty_match_word_boundary() {
    use prog::EmptyLook::*;
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at_index = 5; // Position between "Hello" and ","
    let at = InputAt { pos: at_index, c: Char(u32::MAX), byte: None, len: 0 };
    let empty_instruction = InstEmptyLook { goto: InstPtr(0), look: WordBoundary };

    assert!(input.is_empty_match(at, &empty_instruction));
}

#[test]
fn test_is_empty_match_not_word_boundary() {
    use prog::EmptyLook::*;
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at_index = 7; // Position within "World"
    let at = InputAt { pos: at_index, c: Char(u32::MAX), byte: None, len: 0 };
    let empty_instruction = InstEmptyLook { goto: InstPtr(0), look: NotWordBoundary };

    assert!(input.is_empty_match(at, &empty_instruction));
}

