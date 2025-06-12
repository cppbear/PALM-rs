// Answer 0

fn serialize_newtype_variant_test_success() -> Result<()> {
    struct Formatter;
    
    impl Formatter {
        fn begin_object(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn begin_object_key(&mut self, _: &mut Vec<u8>, _: bool) -> Result<()> {
            Ok(())
        }
        fn end_object_key(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn begin_object_value(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn serialize_str(&self, _: &str) -> Result<()> {
            Ok(())
        }
    }

    let serializer = Serializer {
        formatter: Formatter,
        writer: Vec::new(),
    };

    let result = serializer.serialize_newtype_variant("name", 0, "variant", &123);
    assert!(result.is_ok());
    Ok(())
}

#[test]
#[should_panic]
fn serialize_newtype_variant_test_begin_object_key_err() {
    struct Formatter;

    impl Formatter {
        fn begin_object(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn begin_object_key(&mut self, _: &mut Vec<u8>, _: bool) -> Result<()> {
            Err(Error::io)
        }
        fn end_object_key(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn begin_object_value(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn serialize_str(&self, _: &str) -> Result<()> {
            Ok(())
        }
    }

    let serializer = Serializer {
        formatter: Formatter,
        writer: Vec::new(),
    };

    serializer.serialize_newtype_variant("name", 0, "variant", &123).unwrap();
}

#[test]
fn serialize_newtype_variant_test_value_serialize_err() {
    struct Formatter;

    impl Formatter {
        fn begin_object(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn begin_object_key(&mut self, _: &mut Vec<u8>, _: bool) -> Result<()> {
            Ok(())
        }
        fn end_object_key(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn begin_object_value(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn end_object_value(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
        fn end_object(&mut self, _: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: Formatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn serialize_str(&self, _: &str) -> Result<()> {
            Ok(())
        }
        
        fn serialize_failing(&self) -> Result<()> {
            Err(Error::io)
        }
    }

    let serializer = Serializer {
        formatter: Formatter,
        writer: Vec::new(),
    };

    let result = serializer.serialize_newtype_variant("name", 0, "variant", &serializer.serialize_failing);
    assert!(result.is_err());
}

