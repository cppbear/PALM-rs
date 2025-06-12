// Answer 0

#[derive(Default)]
struct DummyDelegate {
    data: Vec<u8>,
    index: usize,
}

impl DummyDelegate {
    fn new(data: Vec<u8>) -> Self {
        Self { data, index: 0 }
    }

    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        if self.index < self.data.len() {
            Ok(Some(self.data[self.index]))
        } else {
            Ok(None)
        }
    }
}

struct Context {
    delegate: DummyDelegate,
}

impl Context {
    fn new(data: Vec<u8>) -> Self {
        Self {
            delegate: DummyDelegate::new(data),
        }
    }
}

#[test]
fn test_peek_some() {
    let mut context = Context::new(vec![1, 2, 3]);
    assert_eq!(context.delegate.peek().unwrap(), Some(1));
}

#[test]
fn test_peek_none() {
    let mut context = Context::new(vec![]);
    assert_eq!(context.delegate.peek().unwrap(), None);
}

#[test]
fn test_peek_boundary() {
    let mut context = Context::new(vec![1, 2, 3]);
    context.delegate.index = 3; // simulating the delegate reaching the end
    assert_eq!(context.delegate.peek().unwrap(), None);
}

