// Answer 0

#[test]
fn test_serialize_struct_err() {
    struct TestDelegate;

    impl TestDelegate {
        fn serialize_struct(&self, _name: &'static str, _len: usize) -> Result<TestSerializeStruct, &str> {
            Ok(TestSerializeStruct)
        }
    }

    struct TestSerializeStruct;

    impl TestSerializeStruct {
        fn serialize_field(&mut self, _tag: &str, _variant_name: &str) -> Result<(), &str> {
            Err("serialization error")
        }
    }

    struct TestSerializer {
        delegate: TestDelegate,
        tag: &'static str,
        variant_name: &'static str,
    }

    impl TestSerializer {
        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<TestSerializeStruct, &str> {
            let mut state = self.delegate.serialize_struct(name, len + 1)?;
            state.serialize_field(self.tag, self.variant_name)?;
            Ok(state)
        }
    }

    let serializer = TestSerializer {
        delegate: TestDelegate,
        tag: "test_tag",
        variant_name: "test_variant",
    };

    let result = serializer.serialize_struct("test_struct", 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "serialization error");
}

