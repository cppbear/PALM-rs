// Answer 0

#[test]
fn test_new_valid_range() {
    let range = new('\u{0000}', '\u{0001}');
}

#[test]
fn test_new_identical_range() {
    let range = new('\u{0020}', '\u{0020}');
}

#[test]
fn test_new_range_with_unicode_max() {
    let range = new('\u{10FFFF}', '\u{10FFFF}');
}

#[test]
fn test_new_reverse_ordered_range() {
    let range = new('\u{0002}', '\u{0001}');
}

#[test]
fn test_new_large_range() {
    let range = new('\u{10000}', '\u{10001}');
}

#[test]
#[should_panic]
fn test_new_empty_range() {
    let range = new('\u{0001}', '\u{0000}');
}

#[test]
fn test_new_full_range() {
    let range = new('\u{0000}', '\u{10FFFF}');
}

#[test]
fn test_new_non_contiguous() {
    let range1 = new('\u{0030}', '\u{0039}');
    let range2 = new('\u{0041}', '\u{005A}');
}

