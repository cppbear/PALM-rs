// Answer 0

#[test]
fn test_protocol_http_len() {
    #[derive(Copy, Clone, Debug)]
    enum Protocol {
        Http,
        Https,
    }

    impl Protocol {
        fn len(&self) -> usize {
            match *self {
                Protocol::Http => 4,
                Protocol::Https => 5,
            }
        }
    }

    let protocol_http = Protocol::Http;
    assert_eq!(protocol_http.len(), 4);
}

#[test]
fn test_protocol_https_len() {
    #[derive(Copy, Clone, Debug)]
    enum Protocol {
        Http,
        Https,
    }

    impl Protocol {
        fn len(&self) -> usize {
            match *self {
                Protocol::Http => 4,
                Protocol::Https => 5,
            }
        }
    }

    let protocol_https = Protocol::Https;
    assert_eq!(protocol_https.len(), 5);
}

