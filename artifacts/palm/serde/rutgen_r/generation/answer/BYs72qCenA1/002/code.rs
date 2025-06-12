// Answer 0

#[test]
fn test_visit_map_with_valid_data() {
    use serde::de::{MapAccess, Visitor};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    struct MockMapAccess {
        items: Vec<(String, String)>,
        index: usize,
        tag_name: String,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            if self.index < self.items.len() {
                let key = self.items[self.index].0.clone();
                self.index += 1;
                if key == self.tag_name {
                    Ok(Some(TagOrContent::Tag))
                } else {
                    Ok(Some(TagOrContent::Content(key)))
                }
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            if self.index > 0 {
                let value = self.items[self.index - 1].1.clone();
                serde_json::from_str(&value).map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::custom("No more values"))
            }
        }

        fn size_hint(&self) -> Option<(usize, Option<usize>)> {
            Some((self.items.len(), Some(self.items.len())))
        }
    }

    #[derive(Debug)]
    struct Content {
        // Dummy content structure
    }

    #[derive(Debug)]
    enum TagOrContent {
        Tag,
        Content(String),
    }

    struct MyVisitor {
        tag_name: String,
    }

    impl MyVisitor {
        fn new(tag_name: String) -> Self {
            MyVisitor { tag_name }
        }

        fn visit_map<M>(self, mut map: M) -> Result<(Content, Content), M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut tag = None;
            let mut vec = Vec::<(Content, Content)>::new();
            while let Some(k) = tri!(map.next_key_seed(TagOrContentVisitor::new(self.tag_name.clone()))) {
                match k {
                    TagOrContent::Tag => {
                        if tag.is_some() {
                            return Err(serde::de::Error::duplicate_field(self.tag_name.clone()));
                        }
                        tag = Some(tri!(map.next_value()));
                    }
                    TagOrContent::Content(k) => {
                        let v = tri!(map.next_value());
                        vec.push((Content {}, Content {}));
                    }
                }
            }
            match tag {
                None => Err(serde::de::Error::missing_field(self.tag_name.clone())),
                Some(tag) => Ok((Content {}, Content {})),
            }
        }
    }

    let items = vec![
        (String::from("tag"), String::from("{}")), // Mimic a valid tag
        (String::from("key1"), String::from("{}")), // Mimic a content
        (String::from("key2"), String::from("{}")), // Mimic a content
    ];

    let mock_map = MockMapAccess {
        items,
        index: 0,
        tag_name: String::from("tag"),
    };

    let visitor = MyVisitor::new(String::from("tag"));
    let result = visitor.visit_map(mock_map);
    assert!(result.is_ok());
}

