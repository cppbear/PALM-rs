// Answer 0

#[derive(Debug)]
struct MockError;

impl MockError {
    fn new() -> Self {
        MockError
    }
}

#[derive(Debug)]
struct MockAst {
    span: usize,
}

#[derive(Debug)]
struct MockLiteral;

impl MockLiteral {
    fn new() -> Self {
        MockLiteral
    }
}

#[derive(Debug)]
enum MockLiteralEnum {
    Byte(u8),
    Unicode(char),
}

struct MockSelf;

impl MockSelf {
    fn literal_to_char(&self, ast: &MockAst) -> Result<MockLiteralEnum, MockError> {
        // Simulating handling to return Unicode character
        Ok(MockLiteralEnum::Unicode('é'))  // A Unicode character greater than 0x7F
    }

    fn error(&self, _span: usize, _kind: ()) -> Result<u8, MockError> {
        Err(MockError::new())
    }

    fn class_literal_byte(&self, ast: &MockAst) -> Result<u8, MockError> {
        match self.literal_to_char(ast)? {
            MockLiteralEnum::Byte(byte) => Ok(byte),
            MockLiteralEnum::Unicode(ch) => {
                if ch <= 0x7F as char {
                    Ok(ch as u8)
                } else {
                    // We can't feasibly support Unicode in
                    // byte oriented classes. Byte classes don't
                    // do Unicode case folding.
                    Err(self.error(ast.span, ()))
                }
            }
        }
    }
}

#[test]
fn test_class_literal_byte_unicode_not_allowed() {
    let mock_self = MockSelf;
    let ast = MockAst { span: 1 };

    let result = mock_self.class_literal_byte(&ast);
    
    match result {
        Err(_) => {}
        _ => panic!("Expected an error due to Unicode being not allowed."),
    }
}

