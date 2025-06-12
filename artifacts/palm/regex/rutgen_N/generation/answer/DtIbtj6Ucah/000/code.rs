// Answer 0

#[derive(Debug)]
struct MyRegex;

impl MyRegex {
    fn literal_to_char(&self, _ast: &ast::Literal) -> Result<hir::Literal> {
        // Mock implementation for the test
        Ok(hir::Literal::Byte(65)) // Represents 'A'
    }

    fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
        // Mock implementation for the test
        Error::new("Unicode not allowed")
    }

    fn class_literal_byte(&self, ast: &ast::Literal) -> Result<u8> {
        match self.literal_to_char(ast)? {
            hir::Literal::Byte(byte) => Ok(byte),
            hir::Literal::Unicode(ch) => {
                if ch <= 0x7F as char {
                    Ok(ch as u8)
                } else {
                    Err(self.error(ast.span, ErrorKind::UnicodeNotAllowed))
                }
            }
        }
    }
}

#[test]
fn test_class_literal_byte_byte() {
    let regex = MyRegex;
    let ast = ast::Literal::Byte(65); // 'A'
    assert_eq!(regex.class_literal_byte(&ast).unwrap(), 65);
}

#[test]
fn test_class_literal_byte_unicode_valid() {
    let regex = MyRegex;
    let ast = ast::Literal::Unicode('C'); // 'C'
    assert_eq!(regex.class_literal_byte(&ast).unwrap(), 67);
}

#[test]
#[should_panic(expected = "Unicode not allowed")]
fn test_class_literal_byte_unicode_invalid() {
    let regex = MyRegex;
    let ast = ast::Literal::Unicode('Ω'); // Out of byte range
    regex.class_literal_byte(&ast).unwrap();
}

