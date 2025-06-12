// Answer 0

#[test]
fn test_serialize_struct() {
    struct TestSerializer {
        output: Vec<String>,
        error: Option<String>,
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.output.push(format!("Bool: {}", v));
            Ok(())
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.output.push(format!("U8: {}", v));
            Ok(())
        }

        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push(format!("Str: {}", v));
            Ok(())
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.output.push("None".to_string());
            Ok(())
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.output.push("Unit".to_string());
            Ok(())
        }

        fn serialize_newtype_struct(self, _name: &'static str, _value: &Self::Ok) -> Result<Self::Ok, Self::Error> {
            self.output.push("NewtypeStruct".to_string());
            Ok(())
        }

        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self, Self::Error> {
            Ok(self)
        }

        fn serialize_field(self, _name: &'static str, _value: &Self::Ok) -> Result<Self::Ok, Self::Error> {
            self.output.push("Field".to_string());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let content = Content::Struct("MyStruct", vec![
        ("field1", Content::Bool(true)),
        ("field2", Content::U8(255)),
        ("field3", Content::String("hello".to_string())),
    ]);

    let mut serializer = TestSerializer { output: vec![], error: None };

    let result = content.serialize(serializer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());

    assert_eq!(serializer.output.len(), 4);
    assert_eq!(serializer.output[0], "Str: MyStruct");
    assert_eq!(serializer.output[1], "Field");
    assert_eq!(serializer.output[2], "Field");
    assert_eq!(serializer.output[3], "Field");
}

