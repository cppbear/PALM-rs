// Answer 0

#[test]
fn test_peek_some_ch() {
    struct IterWrapper {
        iter: std::vec::IntoIter<Result<u8, std::io::Error>>,
        ch: Option<u8>,
    }

    impl IterWrapper {
        fn new(data: Vec<u8>) -> Self {
            let iter = data.into_iter().map(Ok).collect::<Vec<_>>().into_iter();
            IterWrapper { iter, ch: None }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            match self.ch {
                Some(ch) => Ok(Some(ch)),
                None => match self.iter.next() {
                    Some(Err(err)) => Err(Error::io(err)),
                    Some(Ok(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    None => Ok(None),
                },
            }
        }
    }

    let mut wrapper = IterWrapper::new(vec![1, 2, 3]);
    wrapper.ch = Some(42); // Setting ch to Some(ch)

    let result = wrapper.peek();
    assert_eq!(result, Ok(Some(42)));
}

