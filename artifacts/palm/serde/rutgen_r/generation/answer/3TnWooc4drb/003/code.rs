// Answer 0

#[test]
fn test_serialize_struct_success() {
    struct TestDelegate;

    impl TestDelegate {
        fn serialize_struct(&self, _name: &'static str, _len: usize) -> Result<TestSerializeStruct, &'static str> {
            Ok(TestSerializeStruct)
        }
    }

    struct TestSerializeStruct;

    impl TestSerializeStruct {
        fn serialize_field(&mut self, _tag: &'static str, _variant_name: &'static str) -> Result<(), &'static str> {
            Ok(())
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
        ) -> Result<TestSerializeStruct, &'static str> {
            let mut state = self.delegate.serialize_struct(name, len + 1)?;
            state.serialize_field(self.tag, self.variant_name)?;
            Ok(state)
        }
    }

    let serializer = TestSerializer {
        delegate: TestDelegate,
        tag: "example_tag",
        variant_name: "example_variant",
    };

    let result = serializer.serialize_struct("test_struct", 2);
    assert!(result.is_ok());
}

