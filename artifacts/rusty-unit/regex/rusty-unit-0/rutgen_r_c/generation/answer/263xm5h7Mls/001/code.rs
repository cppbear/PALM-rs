// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    struct TestInput {
        input: CharInput<'static>,
        at: InputAt,
        empty: InstEmptyLook,
    }

    let test_cases = vec![
        TestInput {
            input: CharInput(b"hello"),
            at: InputAt { pos: 2, c: Char(101), byte: Some(b'l'), len: 5 },
            empty: InstEmptyLook {
                goto: InstPtr,
                look: prog::EmptyLook::NotWordBoundaryAscii,
            },
        },
        TestInput {
            input: CharInput(b"hello!"),
            at: InputAt { pos: 5, c: Char(33), byte: Some(b'!'), len: 6 },
            empty: InstEmptyLook {
                goto: InstPtr,
                look: prog::EmptyLook::NotWordBoundaryAscii,
            },
        },
        TestInput {
            input: CharInput(b"1234"),
            at: InputAt { pos: 2, c: Char(50), byte: Some(b'2'), len: 4 },
            empty: InstEmptyLook {
                goto: InstPtr,
                look: prog::EmptyLook::NotWordBoundaryAscii,
            },
        },
        TestInput {
            input: CharInput(b""),
            at: InputAt { pos: 0, c: Char(0), byte: None, len: 0 },
            empty: InstEmptyLook {
                goto: InstPtr,
                look: prog::EmptyLook::NotWordBoundaryAscii,
            },
        },
    ];

    for case in test_cases {
        let result = case.input.is_empty_match(case.at, &case.empty);
        // for positions with word characters, the result should be false
        // for positions without word characters, the result should be true
        if case.at.pos == 2 && case.at.c.is_word_byte() {
            assert!(!result);
        } else if case.at.pos == 5 && case.at.c.is_word_byte() {
            assert!(result);
        } else {
            // Position is at the start or input is empty
            assert!(result);
        }
    }
}

