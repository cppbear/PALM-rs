// Answer 0

#[derive(Debug)]
struct MyEnum {
    type_name: &'static str,
    variant_name: &'static str,
}

impl MyEnum {
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "unit variant {}::{}",
            self.type_name, self.variant_name
        )
    }
}

#[test]
fn test_expecting() {
    let my_enum = MyEnum {
        type_name: "MyType",
        variant_name: "MyVariant",
    };
    
    let mut buffer = String::new();
    let result = my_enum.expecting(&mut std::fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "unit variant MyType::MyVariant");
}

