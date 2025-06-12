// Answer 0

#[test]
fn test_is_empty_match_start_line() {
    let input_data = CharInput(&b"hello\nworld"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::StartLine,
    };
    let at = InputAt {
        pos: 0,
        c: Char(104), // 'h'
        byte: Some(b'h'),
        len: 11,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_line() {
    let input_data = CharInput(&b"hello\nworld"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::EndLine,
    };
    let at = InputAt {
        pos: 10,
        c: Char(108), // 'l'
        byte: Some(b'l'),
        len: 11,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_start_text() {
    let input_data = CharInput(&b"hello"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::StartText,
    };
    let at = InputAt {
        pos: 0,
        c: Char(104), // 'h'
        byte: Some(b'h'),
        len: 5,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_text() {
    let input_data = CharInput(&b"hello"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::EndText,
    };
    let at = InputAt {
        pos: 5,
        c: Char(0), // End of input
        byte: None,
        len: 5,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary() {
    let input_data = CharInput(&b"hello world"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::WordBoundary,
    };
    let at = InputAt {
        pos: 5,
        c: Char(32), // ' '
        byte: Some(b' '),
        len: 11,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary() {
    let input_data = CharInput(&b"hello!"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::NotWordBoundary,
    };
    let at = InputAt {
        pos: 5,
        c: Char(33), // '!'
        byte: Some(b'!'),
        len: 6,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii() {
    let input_data = CharInput(&b"hello"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::WordBoundaryAscii,
    };
    let at = InputAt {
        pos: 5,
        c: Char(0), // End of input
        byte: None,
        len: 5,
    };
    input_data.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    let input_data = CharInput(&b"hello1"[..]);
    let empty = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::NotWordBoundaryAscii,
    };
    let at = InputAt {
        pos: 5,
        c: Char(49), // '1'
        byte: Some(b'1'),
        len: 6,
    };
    input_data.is_empty_match(at, &empty);
}

