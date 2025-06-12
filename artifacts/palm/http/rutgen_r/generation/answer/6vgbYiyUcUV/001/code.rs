// Answer 0

#[derive(Debug)]
struct Method(char); // Basic struct to represent a method

impl Method {
    pub fn is_safe(&self) -> bool {
        matches!(self.0, 'G' | 'H' | 'O' | 'T') // Assuming char representations for methods
    }
}

#[test]
fn test_is_safe_not_safe() {
    // Testing a case where none of the safe methods are matched
    let method_not_safe = Method('P'); // Using a method that is not safe
    assert_eq!(method_not_safe.is_safe(), false);
}

#[test]
fn test_is_safe_boundary_conditions() {
    // Testing for other characters that should not be considered safe
    let method_not_safe_1 = Method('D'); // Not safe
    let method_not_safe_2 = Method('X'); // Not safe
    let method_not_safe_3 = Method('F'); // Not safe
    assert_eq!(method_not_safe_1.is_safe(), false);
    assert_eq!(method_not_safe_2.is_safe(), false);
    assert_eq!(method_not_safe_3.is_safe(), false);
}

