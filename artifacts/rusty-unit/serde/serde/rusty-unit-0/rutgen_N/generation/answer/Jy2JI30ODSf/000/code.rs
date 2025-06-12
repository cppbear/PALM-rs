// Answer 0

#[derive(Debug)]
struct Content {
    seq: Vec<ContentElement>,
}

#[derive(Debug)]
struct ContentElement;

impl Content {
    fn Seq(vec: Vec<ContentElement>) -> Self {
        Content { seq: vec }
    }
}

trait SeqAccess<'de> {
    type Error;
    fn size_hint(&self) -> (Option<usize>, Option<usize>);
    fn next_element(&mut self) -> Result<Option<ContentElement>, Self::Error>;
}

#[derive(Default)]
struct TestVisitor {
    elements: Vec<ContentElement>,
    index: usize,
}

impl<'de> SeqAccess<'de> for TestVisitor {
    type Error = &'static str;

    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (Some(self.elements.len()), Some(self.elements.len()))
    }

    fn next_element(&mut self) -> Result<Option<ContentElement>, Self::Error> {
        if self.index < self.elements.len() {
            let elem = self.elements[self.index].clone();
            self.index += 1;
            Ok(Some(elem))
        } else {
            Ok(None)
        }
    }
}

fn visit_seq<V>(mut visitor: V) -> Result<Content, V::Error>
where
    V: SeqAccess<'de>,
{
    let mut vec = Vec::<Content>::with_capacity(visitor.size_hint().0.unwrap());
    while let Some(e) = visitor.next_element()? {
        vec.push(e);
    }
    Ok(Content::Seq(vec))
}

#[test]
fn test_visit_seq_with_no_elements() {
    let visitor = TestVisitor {
        elements: vec![],
        index: 0,
    };
    let result = visit_seq(visitor);
    assert_eq!(result.unwrap().seq.len(), 0);
}

#[test]
fn test_visit_seq_with_multiple_elements() {
    let elements = vec![ContentElement, ContentElement];
    let visitor = TestVisitor {
        elements,
        index: 0,
    };
    let result = visit_seq(visitor).unwrap();
    assert_eq!(result.seq.len(), 2);
}

