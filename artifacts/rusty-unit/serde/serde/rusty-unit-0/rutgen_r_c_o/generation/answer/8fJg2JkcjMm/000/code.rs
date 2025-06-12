// Answer 0

#[derive(Debug)]
struct MockVisitor {
    value: Option<String>,
}

impl<'de> Visitor<'de> for MockVisitor {
    type Value = String;

    fn visit_bool(self, v: bool) -> Result<Self::Value, E> {
        self.value = Some(format!("Bool: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_u8(self, v: u8) -> Result<Self::Value, E> {
        self.value = Some(format!("U8: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_u16(self, v: u16) -> Result<Self::Value, E> {
        self.value = Some(format!("U16: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_u32(self, v: u32) -> Result<Self::Value, E> {
        self.value = Some(format!("U32: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_u64(self, v: u64) -> Result<Self::Value, E> {
        self.value = Some(format!("U64: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_i8(self, v: i8) -> Result<Self::Value, E> {
        self.value = Some(format!("I8: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_i16(self, v: i16) -> Result<Self::Value, E> {
        self.value = Some(format!("I16: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_i32(self, v: i32) -> Result<Self::Value, E> {
        self.value = Some(format!("I32: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_i64(self, v: i64) -> Result<Self::Value, E> {
        self.value = Some(format!("I64: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_f32(self, v: f32) -> Result<Self::Value, E> {
        self.value = Some(format!("F32: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_f64(self, v: f64) -> Result<Self::Value, E> {
        self.value = Some(format!("F64: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_char(self, v: char) -> Result<Self::Value, E> {
        self.value = Some(format!("Char: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_str(self, v: &str) -> Result<Self::Value, E> {
        self.value = Some(format!("Str: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value, E> {
        self.value = Some(format!("BorrowedStr: {}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_bytes(self, v: &[u8]) -> Result<Self::Value, E> {
        self.value = Some(format!("Bytes: {:?}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_borrowed_bytes(self, v: &'de [u8]) -> Result<Self::Value, E> {
        self.value = Some(format!("BorrowedBytes: {:?}", v));
        Ok(self.value.clone().unwrap())
    }

    fn visit_unit(self) -> Result<Self::Value, E> {
        self.value = Some("Unit".to_string());
        Ok(self.value.clone().unwrap())
    }

    fn visit_none(self) -> Result<Self::Value, E> {
        self.value = Some("None".to_string());
        Ok(self.value.clone().unwrap())
    }

    fn visit_some<V: serde::de::Deserialize>(self, deserializer: V) -> Result<Self::Value, E> {
        self.value = Some("Some".to_string());
        Ok(self.value.clone().unwrap())
    }

    fn visit_newtype_struct<V: serde::de::Deserialize>(self, deserializer: V) -> Result<Self::Value, E> {
        self.value = Some("Newtype".to_string());
        Ok(self.value.clone().unwrap())
    }

    fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, E> 
    where 
        V: serde::de::Deserialize {
        self.value = Some("Seq".to_string());
        Ok(self.value.clone().unwrap())
    }

    fn visit_map<V>(self, visitor: V) -> Result<Self::Value, E> 
    where 
        V: serde::de::Deserialize {
        self.value = Some("Map".to_string());
        Ok(self.value.clone().unwrap())
    }
}

#[test]
fn test_deserialize_any_bool() {
    let content = Content::Bool(true);
    let result = content.deserialize_any(MockVisitor { value: None });
    assert_eq!(result.unwrap(), "Bool: true");
}

#[test]
fn test_deserialize_any_u8() {
    let content = Content::U8(10);
    let result = content.deserialize_any(MockVisitor { value: None });
    assert_eq!(result.unwrap(), "U8: 10");
}

#[test]
fn test_deserialize_any_string() {
    let content = Content::String("test".to_string());
    let result = content.deserialize_any(MockVisitor { value: None });
    assert_eq!(result.unwrap(), "Str: test");
}

#[test]
fn test_deserialize_any_unit() {
    let content = Content::Unit;
    let result = content.deserialize_any(MockVisitor { value: None });
    assert_eq!(result.unwrap(), "Unit");
}

