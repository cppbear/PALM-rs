// Answer 0

#[test]
fn test_program_empty_look() {
    let captures = vec![Some("capture1".to_string()), None];
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);
    let prefixes = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked {},
        lcs: FreqyPacked {},
        matcher: Matcher {},
    };
    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 65, end: 90 }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::WordBoundary }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures,
        capture_name_idx: Arc::new(capture_name_idx),
        start: 1,
        byte_classes: vec![0, 1, 2],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: true,
        prefixes,
        dfa_size_limit: 100,
    };
    let mut output = String::new();
    let _ = write!(output, "{:?}", program);
}

#[test]
#[should_panic]
fn test_program_empty_look_panic() {
    let captures = vec![Some("capture1".to_string()), None];
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);
    let prefixes = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked {},
        lcs: FreqyPacked {},
        matcher: Matcher {},
    };
    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 0, look: EmptyLook::EndLine }),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartText }),
            Inst::Match(1),
        ],
        matches: vec![1],
        captures,
        capture_name_idx: Arc::new(capture_name_idx),
        start: 0,
        byte_classes: vec![0, 1, 2],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes,
        dfa_size_limit: 100,
    };
    let mut output = String::new();
    let _ = write!(output, "{:?}", program);
}

