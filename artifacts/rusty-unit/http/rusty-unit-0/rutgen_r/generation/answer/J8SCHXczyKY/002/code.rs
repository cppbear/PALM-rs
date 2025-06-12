// Answer 0

#[derive(Debug)]
struct ExtensionInline(InlineExtension);

#[derive(Debug)]
struct InlineExtension(Vec<u8>);

#[derive(Debug)]
struct Method(ExtensionInline);

#[derive(Debug)]
struct InvalidMethod;

impl InlineExtension {
    fn new(src: &[u8]) -> Result<Self, InvalidMethod> {
        if src.is_empty() {
            Err(InvalidMethod)
        } else {
            Ok(InlineExtension(src.to_vec()))
        }
    }
}

fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {
    let inline = InlineExtension::new(src)?;

    Ok(Method(ExtensionInline(inline)))
}

#[test]
fn test_extension_inline_valid_input() {
    let input: &[u8] = b"valid input";
    let result = extension_inline(input);
    assert!(result.is_ok());

    if let Ok(method) = result {
        assert_eq!(method.0.0, input.to_vec());
    }
}

#[test]
fn test_extension_inline_empty_input() {
    let input: &[u8] = b"";
    let result = extension_inline(input);
    assert!(result.is_err());
}

#[test]
fn test_extension_inline_edge_case() {
    let input: &[u8] = b" ";
    let result = extension_inline(input);
    assert!(result.is_ok());

    if let Ok(method) = result {
        assert_eq!(method.0.0, input.to_vec());
    }
}

