// Answer 0

#[derive(Debug)]
struct Get;
#[derive(Debug)]
struct Put;
#[derive(Debug)]
struct Post;
#[derive(Debug)]
struct Head;
#[derive(Debug)]
struct Patch;
#[derive(Debug)]
struct Trace;
#[derive(Debug)]
struct Delete;
#[derive(Debug)]
struct Options;
#[derive(Debug)]
struct Connect;

#[derive(Debug)]
struct Method(TraitMethod);
#[derive(Debug)]
enum TraitMethod {
    Get(Get),
    Put(Put),
    Post(Post),
    Head(Head),
    Patch(Patch),
    Trace(Trace),
    Delete(Delete),
    Options(Options),
    Connect(Connect),
    ExtensionAllocated(AllocatedExtension),
}

#[derive(Debug)]
struct InvalidMethod;

impl InvalidMethod {
    fn new() -> Self {
        InvalidMethod {}
    }
}

#[derive(Debug)]
struct AllocatedExtension(Vec<u8>);

impl AllocatedExtension {
    fn new(src: &[u8]) -> Result<Self, InvalidMethod> {
        Ok(AllocatedExtension(src.to_vec()))
    }
}

#[derive(Debug)]
struct InlineExtension;

impl InlineExtension {
    const MAX: usize = 8;
}

impl Method {
    fn extension_inline(src: &[u8]) -> Result<Self, InvalidMethod> {
        Ok(Method(TraitMethod::ExtensionAllocated(AllocatedExtension::new(src)?)))
    }
}

#[test]
fn test_from_bytes_invalid_post() {
    let input = b"POSTE"; // len is 4 but does not match b"POST"
    let result = from_bytes(input);
    assert!(result.is_ok()); // should proceed to inline extension
}

#[test]
fn test_from_bytes_non_matching_post() {
    let input = b"XXXX"; // len is 4 and invalid method
    let result = from_bytes(input);
    assert!(result.is_ok()); // should proceed to inline extension
}

