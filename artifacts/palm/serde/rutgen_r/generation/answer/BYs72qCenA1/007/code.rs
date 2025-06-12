// Answer 0

#[derive(Debug)]
struct TestMapAccess {
    keys: Vec<TagOrContent>,
    values: Vec<Content>,
    index: usize,
}

impl<'de> MapAccess<'de> for TestMapAccess {
    type Error = serde::de::Error;

    fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent>, Self::Error> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error> {
        if self.index <= self.values.len() {
            let value = &self.values[self.index - 1];
            self.index += 1;
            Ok(value.clone() as V)
        } else {
            Err(serde::de::Error::custom("value access out of bounds"))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.keys.len(), Some(self.keys.len()))
    }
}

#[derive(Clone, Debug)]
enum TagOrContent {
    Tag,
    Content(Content),
}

#[derive(Clone, Debug)]
struct Content {
    // Assume Content contains necessary fields for testing
}

#[test]
fn test_visit_map_missing_tag() {
    let map = TestMapAccess {
        keys: vec![
            TagOrContent::Content(Content {}),
            TagOrContent::Content(Content {}),
        ],
        values: vec![Content {}, Content {}],
        index: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_tag() {
    let map = TestMapAccess {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag,
            TagOrContent::Content(Content {}),
        ],
        values: vec![Content {}, Content {}],
        index: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

