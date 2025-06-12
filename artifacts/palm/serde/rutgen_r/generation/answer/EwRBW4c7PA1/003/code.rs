// Answer 0

#[derive(Debug)]
struct MockDelegate;

impl MockDelegate {
    fn serialize_map(&self, _: Option<usize>) -> Result<MockMap, &'static str> {
        Ok(MockMap::new())
    }
}

struct MockMap {
    entries: Vec<(String, String)>,
}

impl MockMap {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }
    
    fn serialize_entry(&mut self, key: String, value: String) -> Result<(), &'static str> {
        self.entries.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<(), &'static str> {
        Ok(())
    }
}

struct MyStruct<'a> {
    delegate: MockDelegate,
    tag: String,
    variant_name: String,
}

impl MyStruct<'_> {
    fn serialize_unit_struct(self) -> Result<(), &'static str> {
        let mut map = self.delegate.serialize_map(Some(1))?;
        map.serialize_entry(self.tag, self.variant_name)?;
        map.end()
    }
}

#[test]
fn test_serialize_unit_struct_success() {
    let my_struct = MyStruct {
        delegate: MockDelegate,
        tag: "my_tag".to_string(),
        variant_name: "my_variant".to_string(),
    };

    assert_eq!(my_struct.serialize_unit_struct(), Ok(()));
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_delegate_fail() {
    struct FailingDelegate;

    impl FailingDelegate {
        fn serialize_map(&self, _: Option<usize>) -> Result<MockMap, &'static str> {
            Err("failed to serialize map")
        }
    }

    struct FailingStruct<'a> {
        delegate: FailingDelegate,
        tag: String,
        variant_name: String,
    }

    impl FailingStruct<'_> {
        fn serialize_unit_struct(self) -> Result<(), &'static str> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_entry(self.tag, self.variant_name)?;
            map.end()
        }
    }

    let failing_struct = FailingStruct {
        delegate: FailingDelegate,
        tag: "some_tag".to_string(),
        variant_name: "some_variant".to_string(),
    };

    failing_struct.serialize_unit_struct().unwrap();
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_entry_fail() {
    struct FailingMap {
        entries: Vec<(String, String)>,
    }

    impl FailingMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
        
        fn serialize_entry(&mut self, _: String, _: String) -> Result<(), &'static str> {
            Err("failed to serialize entry")
        }

        fn end(self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    struct MyStructWithFailingMap<'a> {
        delegate: MockDelegate,
        tag: String,
        variant_name: String,
    }

    impl MyStructWithFailingMap<'_> {
        fn serialize_unit_struct(self) -> Result<(), &'static str> {
            let mut map = self.delegate.serialize_map(Some(1))?;
            map.serialize_entry(self.tag.clone(), self.variant_name.clone())?;
            map.end()
        }
    }

    let my_struct = MyStructWithFailingMap {
        delegate: MockDelegate,
        tag: "my_tag".to_string(),
        variant_name: "my_variant".to_string(),
    };

    my_struct.serialize_unit_struct().unwrap();
}

