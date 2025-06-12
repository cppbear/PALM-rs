// Answer 0

#[derive(Debug)]
struct TypeTag {
    name: String,
}

impl TypeTag {
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a type tag `{}` or any other value", self.name)
    }
}

#[test]
fn test_expecting() {
    let type_tag = TypeTag {
        name: "example".to_string(),
    };
    
    let mut output = String::new();
    let result = type_tag.expecting(&mut output).unwrap();
    
    assert_eq!(output, "a type tag `example` or any other value");
}

#[test]
fn test_expecting_empty_name() {
    let type_tag = TypeTag {
        name: "".to_string(),
    };
    
    let mut output = String::new();
    let result = type_tag.expecting(&mut output).unwrap();
    
    assert_eq!(output, "a type tag `` or any other value");
}

