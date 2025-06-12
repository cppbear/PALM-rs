// Answer 0

#[derive(Default)]
struct MockFormatter;

impl MockFormatter {
    fn begin_object(&self, _writer: &mut ()) -> Result<(), ()> {
        Err(())
    }

    fn begin_object_key(&self, _writer: &mut (), _first: bool) -> Result<(), ()> {
        Ok(())
    }

    fn end_object_key(&self, _writer: &mut ()) -> Result<(), ()> {
        Ok(())
    }

    fn begin_object_value(&self, _writer: &mut ()) -> Result<(), ()> {
        Ok(())
    }

    fn end_object_value(&self, _writer: &mut ()) -> Result<(), ()> {
        Ok(())
    }

    fn end_object(&self, _writer: &mut ()) -> Result<(), ()> {
        Ok(())
    }
}

struct MockSer {
    formatter: MockFormatter,
    writer: (),
}

impl MockSer {
    pub fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), ()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.formatter.begin_object(&mut self.writer)?;
        self.formatter.begin_object_key(&mut self.writer, true)?;
        self.serialize_str(variant)?;
        self.formatter.end_object_key(&mut self.writer)?;
        self.formatter.begin_object_value(&mut self.writer)?;
        value.serialize(&mut serde_json::Serializer::new(self.writer))?;
        self.formatter.end_object_value(&mut self.writer)?;
        self.formatter.end_object(&mut self.writer)?;
        Ok(())
    }

    fn serialize_str(&self, _value: &'static str) -> Result<(), ()> {
        Ok(())
    }
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_failure() {
    let mock_ser = MockSer::default();
    let result = mock_ser.serialize_newtype_variant("test_name", 0, "test_variant", &"test_value");
    assert!(result.is_err());
}

