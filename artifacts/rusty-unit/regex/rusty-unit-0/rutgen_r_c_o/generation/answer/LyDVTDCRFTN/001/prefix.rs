// Answer 0

#[test]
fn test_start_range_lower_bound() {
    let range = ClassUnicodeRange::new('\u{0000}', '\u{0001}');
    range.start();
}

#[test]
fn test_start_range_middle() {
    let range = ClassUnicodeRange::new('\u{007F}', '\u{00FF}');
    range.start();
}

#[test]
fn test_start_range_upper_bound() {
    let range = ClassUnicodeRange::new('\u{10FFFF}', '\u{10FFFF}');
    range.start();
}

#[test]
fn test_start_range_same_start_end() {
    let range = ClassUnicodeRange::new('\u{1234}', '\u{1234}');
    range.start();
}

#[test]
fn test_start_range_non_contiguous() {
    let range1 = ClassUnicodeRange::new('\u{0000}', '\u{007F}');
    let range2 = ClassUnicodeRange::new('\u{0080}', '\u{00FF}');
    range1.start();
    range2.start();
}

