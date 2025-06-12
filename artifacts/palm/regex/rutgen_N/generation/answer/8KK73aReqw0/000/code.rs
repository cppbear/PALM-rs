// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::collections::HashMap;

    #[test]
    fn test_program_new() {
        let program = new();
        
        assert_eq!(program.insts.len(), 0);
        assert_eq!(program.matches.len(), 0);
        assert_eq!(program.captures.len(), 0);
        assert_eq!(Arc::strong_count(&program.capture_name_idx), 1);
        assert_eq!(program.start, 0);
        assert_eq!(program.byte_classes, vec![0; 256]);
        assert!(program.only_utf8);
        assert!(!program.is_bytes);
        assert!(!program.is_dfa);
        assert!(!program.is_reverse);
        assert!(!program.is_anchored_start);
        assert!(!program.is_anchored_end);
        assert!(!program.has_unicode_word_boundary);
        assert_eq!(program.dfa_size_limit, 2 * (1<<20));
    }
}

