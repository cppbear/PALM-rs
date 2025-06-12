// Answer 0

#[test]
fn test_serialize_element() {
    use serde::ser::{Serializer, SerializeSeq};
    use serde::Serialize;
    use serde_json::ser::Serializer as JsonSerializer;

    struct MySeqSerializer {
        elements: Vec<String>,
    }

    impl MySeqSerializer {
        fn new() -> Self {
            MySeqSerializer { elements: Vec::new() }
        }
    }

    impl Serializer for MySeqSerializer {
        type Ok = ();
        type Error = serde_json::Error;
        type SerializeSeq = MySerializeSeq;

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Ok(MySerializeSeq { serializer: self, len })
        }

        // Other methods of Serializer would need real implementation in a full context
    }

    struct MySerializeSeq {
        serializer: MySeqSerializer,
        len: Option<usize>,
    }

    impl SerializeSeq for MySerializeSeq {
        type Ok = ();
        type Error = serde_json::Error;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.serializer.elements.push(value.serialize(&mut JsonSerializer::new()).unwrap());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut serializer = MySeqSerializer::new();
    let result = serializer.serialize_seq(Some(1));

    assert!(result.is_ok());

    let mut seq = result.unwrap();
    let value = "Hello, World!";
    let element_result = seq.serialize_element(&value);

    assert!(element_result.is_ok());
    assert_eq!(serializer.elements.len(), 1);
    assert_eq!(serializer.elements[0], serde_json::to_string(value).unwrap());
}

