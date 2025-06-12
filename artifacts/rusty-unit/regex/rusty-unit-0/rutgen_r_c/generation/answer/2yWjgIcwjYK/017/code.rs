// Answer 0

#[test]
fn test_program_fmt_multiple_instructions() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::{self, Write};

    let start = 1;
    let insts = vec![
        Inst::Char(InstChar { goto: 1, c: 'a' }), // This will cause pc == start to be true
        Inst::Char(InstChar { goto: 2, c: 'b' }),
        Inst::Char(InstChar { goto: 3, c: 'c' }),
        Inst::Match(0),
    ];
    
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![],
        capture_name_idx,
        start,
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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 100,
    };
    
    let mut output = String::new();
    
    // Call the fmt method directly
    let result = program.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "000 Char('a') (goto: 1)\n001 Char('b') (goto: 2)\n002 Char('c') (goto: 3)\n003 Match(0) (start)\n");
}

#[test]
fn test_program_fmt_single_instruction() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::{self, Write};

    let start = 0;
    let inst = vec![
        Inst::Char(InstChar { goto: 1, c: 'x' }), // Only one instruction
        Inst::Match(0),
    ];
    
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts: inst,
        matches: vec![0],
        captures: vec![],
        capture_name_idx,
        start,
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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 50,
    };
    
    let mut output = String::new();
    
    // Call the fmt method directly
    let result = program.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "000 Char('x') (goto: 1)\n001 Match(0) (start)\n");
}

#[test]
fn test_program_fmt_empty_program() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::{self, Write};

    let start = 0;
    let inst = vec![];
    
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts: inst,
        matches: vec![],
        captures: vec![],
        capture_name_idx,
        start,
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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };
    
    let mut output = String::new();
    
    // Call the fmt method directly
    let result = program.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, ""); // Should be empty for no instructions
}

