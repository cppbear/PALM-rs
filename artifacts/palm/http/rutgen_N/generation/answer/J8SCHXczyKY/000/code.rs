// Answer 0

#[derive(Debug)]
struct InvalidMethod;

struct Method(ExtensionInline);

struct InlineExtension<'a> {
    data: &'a [u8],
}

impl<'a> InlineExtension<'a> {
    fn new(data: &'a [u8]) -> Result<Self, InvalidMethod> {
        if data.is_empty() {
            return Err(InvalidMethod);
        }
        Ok(InlineExtension { data })
    }
}

struct ExtensionInline(InlineExtension<'static>);

#[test]
fn test_extension_inline_valid_input() {
    let input: &[u8] = b"valid";
    let result = extension_inline(input);
    assert!(result.is_ok());

    match result {
        Ok(method) => {
            if let Method(extension) = method {
                assert_eq!(extension.0.data, input);
            }
        }
        _ => panic!("Expected Ok, but got Err"),
    }
}

#[test]
fn test_extension_inline_empty_input() {
    let input: &[u8] = b"";
    let result = extension_inline(input);
    assert!(result.is_err());
}

