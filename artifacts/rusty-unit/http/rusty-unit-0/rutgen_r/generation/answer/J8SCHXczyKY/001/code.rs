// Answer 0

#[derive(Debug)]
struct InvalidMethod;

#[derive(Debug)]
struct Method(ExtensionInline);

#[derive(Debug)]
struct ExtensionInline(InlineExtension);

struct InlineExtension {
    data: Vec<u8>,
}

impl InlineExtension {
    fn new(src: &[u8]) -> Result<Self, InvalidMethod> {
        if src.is_empty() {
            return Err(InvalidMethod);
        }
        Ok(InlineExtension { data: src.to_vec() })
    }
}

fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {
    let inline = InlineExtension::new(src)?;
    Ok(Method(ExtensionInline(inline)))
}

#[test]
fn test_extension_inline_with_valid_input() {
    let input = b"GET";
    let result = extension_inline(input);
    assert!(result.is_ok());
}

#[test]
fn test_extension_inline_with_empty_input() {
    let input: &[u8] = b"";
    let result = extension_inline(input);
    assert!(result.is_err());
}

