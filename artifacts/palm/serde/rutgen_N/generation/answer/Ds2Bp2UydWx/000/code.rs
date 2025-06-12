// Answer 0

#[derive(Debug)]
struct DummyVisitor;

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }

    // Implement other required methods by Visitor trait as no-op
    fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_i8(self, _: i8) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_i16(self, _: i16) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_i32(self, _: i32) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_i64(self, _: i64) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_u8(self, _: u8) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_u16(self, _: u16) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_u32(self, _: u32) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_u64(self, _: u64) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_f32(self, _: f32) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_f64(self, _: f64) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_str(self, _: &str) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_none(self) -> Result<Self::Value, serde::de::Error> { Ok(()) }
    fn visit_some<D>(self, _: D) -> Result<Self::Value, serde::de::Error>
    where
        D: Deserializer<'de>, { Ok(()) }
}

#[test]
fn test_deserialize_ignored_any() {
    let visitor = DummyVisitor;
    let result: Result<(), serde::de::Error> = deserialize_ignored_any(visitor);
    assert!(result.is_ok());
}

