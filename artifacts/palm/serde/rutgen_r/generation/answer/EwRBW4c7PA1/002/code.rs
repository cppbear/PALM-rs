// Answer 0

#[test]
fn test_serialize_unit_struct_err_on_serialize_entry() {
    struct TestDelegate;

    impl TestDelegate {
        fn serialize_map(&self, _: Option<usize>) -> Result<TestMap, &'static str> {
            Ok(TestMap)
        }
    }

    struct TestMap;

    impl TestMap {
        fn serialize_entry(&mut self, _: &'static str, _: &'static str) -> Result<(), &'static str> {
            Err("serialize_entry error")
        }

        fn end(self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    struct TestSerializer {
        delegate: TestDelegate,
        tag: &'static str,
        variant_name: &'static str,
    }

    impl TestSerializer {
        fn serialize_unit_struct(self) -> Result<(), &'static str> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_entry(self.tag, self.variant_name)?;
            map.end()
        }
    }

    let serializer = TestSerializer {
        delegate: TestDelegate,
        tag: "tag_value",
        variant_name: "variant_name_value",
    };

    let result = serializer.serialize_unit_struct();
    assert!(result.is_err());
    assert_eq!(result.err(), Some("serialize_entry error"));
}

