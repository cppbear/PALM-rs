// Answer 0

#[derive(Debug)]
struct Content {
    kind: ContentKind,
}

#[derive(Debug)]
enum ContentKind {
    UnitStruct(&'static str),
}

struct Serializer;

impl Serializer {
    fn serialize_unit_struct(self, name: &'static str) -> Result<Content, &'static str> {
        Ok(Content { kind: ContentKind::UnitStruct(name) })
    }
}

#[test]
fn test_serialize_unit_struct() {
    let serializer = Serializer;
    let result = serializer.serialize_unit_struct("MyUnitStruct");
    
    match result {
        Ok(content) => {
            if let ContentKind::UnitStruct(ref n) = content.kind {
                assert_eq!(n, "MyUnitStruct");
            } else {
                panic!("Unexpected content kind");
            }
        },
        Err(e) => panic!("Expected Ok but got Err: {}", e),
    }
}

#[test]
fn test_serialize_unit_struct_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_unit_struct("");
    
    match result {
        Ok(content) => {
            if let ContentKind::UnitStruct(ref n) = content.kind {
                assert_eq!(n, "");
            } else {
                panic!("Unexpected content kind");
            }
        },
        Err(e) => panic!("Expected Ok but got Err: {}", e),
    }
}

