// Answer 0

#[test]
fn test_case_fold_simple_with_no_matching_range() {
    struct TestStruct {
        start: u32,
        end: u32,
    }

    impl TestStruct {
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

    let my_struct = TestStruct { start: 0x110000, end: 0x10FFFF }; // Unicode range outside valid character range
    let mut ranges: Vec<ClassUnicodeRange> = vec![];

    my_struct.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty()); // Expect no ranges to be added
}

#[test]
fn test_case_fold_simple_with_empty_range() {
    struct TestStruct {
        start: u32,
        end: u32,
    }

    impl TestStruct {
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

    let my_struct = TestStruct { start: 0x007F, end: 0x007F }; // A single character range
    let mut ranges: Vec<ClassUnicodeRange> = vec![];

    my_struct.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty()); // Expect no ranges to be added as it has no simple case mapping
}

#[test]
#[should_panic]
fn test_case_fold_simple_with_invalid_range() {
    struct TestStruct {
        start: u32,
        end: u32,
    }

    impl TestStruct {
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

    let my_struct = TestStruct { start: 0xFFFF, end: 0xFFFF }; // Test invalid range for panic
    let mut ranges: Vec<ClassUnicodeRange> = vec![];

    my_struct.case_fold_simple(&mut ranges); // Expected to panic or fail gracefully
}

