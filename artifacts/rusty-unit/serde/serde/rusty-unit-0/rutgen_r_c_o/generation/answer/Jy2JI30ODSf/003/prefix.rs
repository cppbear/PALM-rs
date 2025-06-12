// Answer 0

use serde::de::{self, Content, SeqAccess, Visitor};
use std::marker::PhantomData;

struct TestSeqAccess<'de> {
    values: Vec<Result<Content<'de>, de::Error>>,
    current: usize,
}

impl<'de> SeqAccess<'de> for TestSeqAccess<'de> {
    type Error = de::Error;

    fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
        if self.current < self.values.len() {
            let value = self.values[self.current].clone();
            self.current += 1;
            value.map(|content| Some(content)).map_err(|err| err)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.values.len() - self.current)
    }
}

#[derive(Debug, Clone)]
struct ContentVisitor<'de> {
    _marker: PhantomData<&'de ()>,
}

impl<'de> Visitor<'de> for ContentVisitor<'de> {
    type Value = Content<'de>;

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(visitor.size_hint().unwrap_or(0));
        while let Some(e) = visitor.next_element()? {
            vec.push(e);
        }
        Ok(Content::Seq(vec))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "an array of contents")
    }
}

#[test]
fn test_visit_seq_success() {
    let test_data = (1..=10)
        .map(|i| Ok(Content::I32(i)))
        .collect::<Vec<_>>();
    let mut seq_access = TestSeqAccess { values: test_data, current: 0 };
    let visitor = ContentVisitor { _marker: PhantomData };
    let _ = visitor.visit_seq(seq_access);
}

#[test]
fn test_visit_seq_err() {
    let test_data = vec![
        Ok(Content::I32(1)),
        Err(de::Error::custom("error")),
        Ok(Content::I32(2)),
    ];
    let mut seq_access = TestSeqAccess { values: test_data, current: 0 };
    let visitor = ContentVisitor { _marker: PhantomData };
    let result = visitor.visit_seq(seq_access);
    let _ = result.unwrap_err(); // ensures an error is returned
} 

#[test]
fn test_visit_seq_empty() {
    let test_data: Vec<Result<Content, de::Error>> = Vec::new();
    let mut seq_access = TestSeqAccess { values: test_data, current: 0 };
    let visitor = ContentVisitor { _marker: PhantomData };
    let _ = visitor.visit_seq(seq_access);
}

#[test]
fn test_visit_seq_some_empty() {
    let test_data = vec![Err(de::Error::custom("error"))];
    let mut seq_access = TestSeqAccess { values: test_data, current: 0 };
    let visitor = ContentVisitor { _marker: PhantomData };
    let result = visitor.visit_seq(seq_access);
    let _ = result.unwrap_err(); // ensures an error is returned
} 

#[test]
fn test_visit_seq_mixed() {
    let test_data = vec![
        Ok(Content::I32(1)),
        Ok(Content::I32(2)),
        Err(de::Error::custom("error")),
        Ok(Content::I32(3)),
    ];
    
    let mut seq_access = TestSeqAccess { values: test_data, current: 0 };
    let visitor = ContentVisitor { _marker: PhantomData };
    let result = visitor.visit_seq(seq_access);
    let _ = result.unwrap_err(); // ensures an error is returned
}

