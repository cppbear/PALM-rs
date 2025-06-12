// Answer 0

#[derive(Debug)]
struct MockVisitor;

impl<'de> serde::de::Visitor<'de> for MockVisitor {
    type Value = Vec<u32>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of u32 values")
    }

    fn visit_seq<V>(self, mut visitor: V) -> std::result::Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(value) = visitor.next_element()? {
            vec.push(value);
        }
        Ok(vec)
    }
}

#[derive(Debug)]
enum Content {
    Seq(Vec<u32>),
    Other,
}

struct Deserializer {
    content: Box<Content>,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
        "Invalid type"
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: serde::de::Visitor<'de>,
    {
        match *self.content {
            Content::Seq(ref v) => visit_content_seq_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

fn visit_content_seq_ref<V>(content: &Vec<u32>, visitor: V) -> Result<V::Value, V::Error>
where
    V: serde::de::Visitor<'de>,
{
    let mut seq_access = content.into_iter();
    let mut value = visitor.visit_seq(&mut seq_access)?;
    Ok(value)
}

#[test]
fn test_deserialize_seq_valid() {
    let content = Content::Seq(vec![1, 2, 3]);
    let deserializer = Deserializer { content: Box::new(content) };
    let mock_visitor = MockVisitor;

    let result: Result<Vec<u32>, &'static str> = deserializer.deserialize_seq(mock_visitor);

    assert_eq!(result, Ok(vec![1, 2, 3]));
}

#[test]
fn test_deserialize_seq_invalid() {
    let content = Content::Other;
    let deserializer = Deserializer { content: Box::new(content) };
    let mock_visitor = MockVisitor;

    let result: Result<Vec<u32>, &'static str> = deserializer.deserialize_seq(mock_visitor);

    assert_eq!(result, Err("Invalid type"));
}

