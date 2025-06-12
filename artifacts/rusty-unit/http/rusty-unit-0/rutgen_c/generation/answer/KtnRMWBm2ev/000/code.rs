// Answer 0

#[test]
fn test_protocol_len_http() {
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

    let http = Protocol::Http;
    assert_eq!(http.len(), 4);
}

#[test]
fn test_protocol_len_https() {
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

    let https = Protocol::Https;
    assert_eq!(https.len(), 5);
}

