// Answer 0

#[derive(Debug)]
struct DummyVisitor;

impl<'de> serde::de::EnumAccess<'de> for DummyVisitor {
    type Error = serde::de::Error;
    type Variant = ();

    fn variant<V>(self) -> Result<(serde::Deserialize, V), Self::Error>
    where
        V: serde::de::Deserializer<'de>,
    {
        Err(serde::de::Error::custom("DummyVisitor error"))
    }
}

#[test]
fn test_visit_enum_err() {
    let visitor = DummyVisitor;

    let result: Result<(), serde::de::Error> = Err(serde::de::Error::custom(
        "untagged and internally tagged enums do not support enum input",
    ));

    assert_eq!(result.unwrap_err().to_string(), "untagged and internally tagged enums do not support enum input");
}

