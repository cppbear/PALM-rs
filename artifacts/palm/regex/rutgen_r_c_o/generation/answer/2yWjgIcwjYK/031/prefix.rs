// Answer 0

#[test]
fn test_program_with_save_instruction_panic() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let insts = vec![
        Inst::Save(InstSave { goto: 1, slot: 0 }),
        Inst::Match(1),
        Inst::Save(InstSave { goto: 3, slot: 2 }),
        Inst::Split(InstSplit { goto1: 4, goto2: 5 }),
        Inst::Char(InstChar { goto: 6, c: 'a' }),
        Inst::Save(InstSave { goto: 7, slot: 4 }),
        Inst::Bytes(InstBytes { goto: 8, start: 0x61, end: 0x7A }),
    ];

    let matches = vec![1];
    let captures = vec![None];
    let capture_name_idx = Arc::new(HashMap::new());
    let dfa_size_limit = 3;

    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false, 
            lcp: FreqyPacked(), 
            lcs: FreqyPacked(), 
            matcher: Matcher(),
        },
        dfa_size_limit,
    };

    let result = program.fmt(&mut std::fmt::Formatter::new());
} 

#[test]
fn test_program_with_goto_out_of_bounds() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let insts = vec![
        Inst::Save(InstSave { goto: 11, slot: 0 }), // This should cause out of bounds panic
        Inst::Char(InstChar { goto: 1, c: 'b' }),
        Inst::Match(0),
    ];

    let matches = vec![0];
    let captures = vec![None];
    let capture_name_idx = Arc::new(HashMap::new());
    let dfa_size_limit = 3;

    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false, 
            lcp: FreqyPacked(), 
            lcs: FreqyPacked(), 
            matcher: Matcher(),
        },
        dfa_size_limit,
    };

    let result = program.fmt(&mut std::fmt::Formatter::new());
} 

#[test]
fn test_program_multiple_save_instructions() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let insts = vec![
        Inst::Save(InstSave { goto: 1, slot: 0 }),
        Inst::Save(InstSave { goto: 2, slot: 1 }),
        Inst::Split(InstSplit { goto1: 3, goto2: 4 }),
        Inst::Match(0),
    ];

    let matches = vec![0];
    let captures = vec![Some("group0".to_string()), Some("group1".to_string())];
    let capture_name_idx = Arc::new(HashMap::new());
    let dfa_size_limit = 5;

    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false, 
            lcp: FreqyPacked(), 
            lcs: FreqyPacked(), 
            matcher: Matcher(),
        },
        dfa_size_limit,
    };

    let result = program.fmt(&mut std::fmt::Formatter::new());
} 

#[test]
fn test_program_with_no_save_instructions() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let insts = vec![
        Inst::Match(0),
        Inst::Split(InstSplit { goto1: 1, goto2: 2 }),
        Inst::Char(InstChar { goto: 3, c: 'c' }),
    ];

    let matches = vec![0];
    let captures = vec![None];
    let capture_name_idx = Arc::new(HashMap::new());
    let dfa_size_limit = 2;

    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false, 
            lcp: FreqyPacked(), 
            lcs: FreqyPacked(), 
            matcher: Matcher(),
        },
        dfa_size_limit,
    };

    let result = program.fmt(&mut std::fmt::Formatter::new());
} 

