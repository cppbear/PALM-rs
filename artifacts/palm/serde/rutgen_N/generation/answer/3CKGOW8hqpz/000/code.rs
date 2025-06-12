// Answer 0

#[test]
fn test_serialize_unit_variant() {
    use serde::ser::{Serializer, Serialize};
    use serde::ser::Error;

    struct TestSerializer {
        entries: Vec<(String, String)>,
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = String;
        type SerializeMap = TestMapSerializer;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(TestMapSerializer {
                serializer: self,
                entries: vec![],
            })
        }
    }

    struct TestMapSerializer {
        serializer: TestSerializer,
        entries: Vec<(String, String)>,
    }

    impl serde::ser::SerializeMap for TestMapSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_entry(&mut self, key: &str, value: &()) -> Result<(), Self::Error> {
            self.entries.push((key.to_string(), "unit".to_string()));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.serializer.entries = self.entries;
            Ok(())
        }
    }

    let serializer = TestSerializer { entries: vec![] };
    let result = serialize_unit_variant(serializer, "Test", 0, "InnerVariant");

    assert!(result.is_ok());
    assert_eq!(serializer.entries.len(), 2);
    assert_eq!(serializer.entries[0].0, "Test");
    assert_eq!(serializer.entries[1].0, "InnerVariant");
}

