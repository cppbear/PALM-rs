// Answer 0

#[test]
fn test_program_fmt_match() {
    use std::fmt::Write;
    use std::collections::HashMap;
    use std::sync::Arc;

    // Setup a Program with a single Match instruction.
    let mut insts = Vec::new();
    insts.push(Inst::Match(0)); // Match instruction at index 0

    // The capture groups are irrelevant for this test, so we can leave them empty.
    let captures = Vec::new();
    let capture_name_idx = Arc::new(HashMap::new());

    let program = Program {
        insts,
        matches: vec![0], // Pointing to the match instruction
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::empty(),
            lcs: FreqyPacked::empty(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let mut output = String::new();
    assert!(program.fmt(&mut output).is_ok());

    // Check if the output is as expected
    let expected_output = "000 Match(0)\n (start)\n";
    assert_eq!(output, expected_output);
}

#[test]
#[should_panic]
fn test_program_fmt_should_panic_on_write_error() {
    use std::fmt::Write;
    use std::collections::HashMap;
    use std::sync::Arc;

    // Setup a Program with a single Match instruction.
    let mut insts = Vec::new();
    insts.push(Inst::Match(0)); // Match instruction at index 0

    // Intentionally use an invalid output format to cause a panic
    let mut output = String::with_capacity(10); // Limit capacity to trigger panic on write

    let captures = Vec::new();
    let capture_name_idx = Arc::new(HashMap::new());

    let program = Program {
        insts,
        matches: vec![0], // Pointing to the match instruction
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::empty(),
            lcs: FreqyPacked::empty(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    // This should panic because the output can't accommodate the write
    let _ = program.fmt(&mut output);
}

