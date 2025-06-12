// Answer 0

#[test]
fn test_matches_out_of_range_start() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert_eq!(inst_bytes.matches(4), false); // self.start <= byte is false
}

#[test]
fn test_matches_exclusive_start() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert_eq!(inst_bytes.matches(5), true); // self.start <= byte is true
}

#[test]
fn test_matches_in_range() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert_eq!(inst_bytes.matches(7), true); // self.start <= byte is true and byte <= self.end is true
}

#[test]
fn test_matches_exclusive_end() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert_eq!(inst_bytes.matches(10), true); // byte <= self.end is true
}

#[test]
fn test_matches_out_of_range_end() {
    let inst_bytes = InstBytes { goto: 0, start: 5, end: 10 };
    assert_eq!(inst_bytes.matches(11), false); // byte <= self.end is false
}

#[test]
fn test_matches_boundary_conditions() {
    let inst_bytes_zero = InstBytes { goto: 0, start: 0, end: 0 };
    assert_eq!(inst_bytes_zero.matches(0), true); // start and end are the same
    assert_eq!(inst_bytes_zero.matches(1), false); // out of range
    
    let inst_bytes_negative = InstBytes { goto: 0, start: 0, end: 255 };
    assert_eq!(inst_bytes_negative.matches(255), true); // at the end
    assert_eq!(inst_bytes_negative.matches(256), false); // out of range
}

