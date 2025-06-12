// Answer 0

#[test]
fn test_deserialize_float_content_i32() {
    struct VisitorImpl {
        value: Option<i32>,
    }

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Option<i32>;

        fn visit_f32(self, _: f32) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i32(self, value: i32) -> Result<Self::Value, E> {
            Ok(Some(value))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, E> {
            Ok(self.value)
        }
        
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
            Ok(self.value)
        }
        
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_none(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D>(self, _: D) -> Result<Self::Value, E>
        where
            D: Deserializer<'de>,
        {
            Ok(self.value)
        }

        fn visit_unit(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    let content = Content::I32(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = VisitorImpl { value: None };

    let result = deserializer.deserialize_float(visitor);

    assert_eq!(result.unwrap(), Some(42));
}

#[test]
fn test_deserialize_float_content_invalid() {
    struct VisitorImpl {
        value: Option<i32>,
    }

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Option<i32>;

        fn visit_f32(self, _: f32) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, E> {
            Ok(self.value)
        }
        
        fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, E> {
            Ok(self.value)
        }

        fn visit_none(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D>(self, _: D) -> Result<Self::Value, E>
        where
            D: Deserializer<'de>,
        {
            Ok(self.value)
        }

        fn visit_unit(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    let content = Content::Char('a');
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = VisitorImpl { value: None };

    let result = deserializer.deserialize_float(visitor);

    assert!(result.is_err());
}

