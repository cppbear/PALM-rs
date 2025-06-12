// Answer 0

#[test]
fn test_c_char_valid_character() {
    struct TestStruct;

    impl TestStruct {
        fn c_char(&mut self, c: char) -> Result<(), String> {
            self.c_class(&[hir::ClassUnicodeRange::new(c, c)])
        }

        fn c_class(&self, _ranges: &[hir::ClassUnicodeRange]) -> Result<(), String> {
            // Dummy implementation for testing
            Ok(())
        }
    }

    let mut instance = TestStruct;
    let result = instance.c_char('a');
    assert_eq!(result, Ok(()));
}

#[test]
fn test_c_char_boundary_character() {
    struct TestStruct;

    impl TestStruct {
        fn c_char(&mut self, c: char) -> Result<(), String> {
            self.c_class(&[hir::ClassUnicodeRange::new(c, c)])
        }

        fn c_class(&self, _ranges: &[hir::ClassUnicodeRange]) -> Result<(), String> {
            // Dummy implementation for testing
            Ok(())
        }
    }

    let mut instance = TestStruct;
    let result = instance.c_char('\0');
    assert_eq!(result, Ok(()));
}

#[test]
fn test_c_char_invalid_character() {
    struct TestStruct;

    impl TestStruct {
        fn c_char(&mut self, c: char) -> Result<(), String> {
            if !c.is_ascii() {
                return Err("Invalid character".to_string());
            }
            self.c_class(&[hir::ClassUnicodeRange::new(c, c)])
        }

        fn c_class(&self, _ranges: &[hir::ClassUnicodeRange]) -> Result<(), String> {
            // Dummy implementation for testing
            Ok(())
        }
    }

    let mut instance = TestStruct;
    let result = instance.c_char('😀'); // Using a non-ASCII character
    assert_eq!(result, Err("Invalid character".to_string()));
}

