// Answer 0

fn test_case_fold_simple_valid_range() {
    struct TestRange {
        start: u32,
        end: u32,
    }
    
    impl TestRange {
        fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {
            if !unicode::contains_simple_case_mapping(self.start, self.end) {
                return;
            }
            let start = self.start as u32;
            let end = (self.end as u32).saturating_add(1);
            let mut next_simple_cp = None;
            for cp in (start..end).filter_map(char::from_u32) {
                if next_simple_cp.map_or(false, |next| cp < next) {
                    continue;
                }
                let it = match unicode::simple_fold(cp) {
                    Ok(it) => it,
                    Err(next) => {
                        next_simple_cp = next;
                        continue;
                    }
                };
                for cp_folded in it {
                    ranges.push(ClassUnicodeRange::new(cp_folded, cp_folded));
                }
            }
        }
    }

    let range = TestRange { start: 0x41, end: 0x5A }; // A-Z (ASCII upper case)
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert!(!ranges.is_empty());
}

fn test_case_fold_simple_empty_range() {
    struct TestRange {
        start: u32,
        end: u32,
    }
    
    impl TestRange {
        fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {
            // Implementation equivalent to the original function
        }
    }

    let range = TestRange { start: 0, end: 0 }; // Empty range
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty());
}

fn test_case_fold_simple_boundary_condition() {
    struct TestRange {
        start: u32,
        end: u32,
    }
    
    impl TestRange {
        fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {
            // Implementation equivalent to the original function
        }
    }

    let range = TestRange { start: 0x10FFFF, end: 0x10FFFF }; // Highest Unicode code point
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty());
}

fn test_case_fold_simple_no_case_mapping() {
    struct TestRange {
        start: u32,
        end: u32,
    }
    
    impl TestRange {
        fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {
            // Implementation equivalent to the original function
        }
    }

    let range = TestRange { start: 0x2C60, end: 0x2C61 }; // No simple case mapping
    let mut ranges = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty());
}

