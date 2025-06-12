// Answer 0

#[test]
fn test_len_http_protocol() {
    struct Protocol {
        is_http: bool,
    }

    impl Protocol {
        fn len(&self) -> usize {
            match self.is_http {
                true => 4,
                false => 5, // This branch won't be triggered in this test.
            }
        }
    }

    let http_protocol = Protocol { is_http: true };
    assert_eq!(http_protocol.len(), 4);
}

#[test]
fn test_len_https_protocol() {
    struct Protocol {
        is_http: bool,
    }

    impl Protocol {
        fn len(&self) -> usize {
            match self.is_http {
                true => 4,
                false => 5,
            }
        }
    }

    let https_protocol = Protocol { is_http: false };
    assert_eq!(https_protocol.len(), 5);
}

