// Answer 0

#[test]
fn test_new_content_serializer() {
    let serializer: ContentSerializer<SomeErrorType> = ContentSerializer::new();
}

#[test]
fn test_new_content_serializer_another_error() {
    let serializer: ContentSerializer<AnotherErrorType> = ContentSerializer::new();
} 

struct SomeErrorType; // Dummy struct implementing Error trait for testing
impl std::fmt::Debug for SomeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SomeErrorType")
    }
}

struct AnotherErrorType; // Another Dummy struct implementing Error trait for testing
impl std::fmt::Debug for AnotherErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AnotherErrorType")
    }
}

