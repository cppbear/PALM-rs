// Answer 0

#[test]
fn test_len_https() {
    struct Protocol {
        kind: ProtocolKind,
    }
    
    enum ProtocolKind {
        Http,
        Https,
    }
    
    impl Protocol {
        pub(super) fn len(&self) -> usize {
            match self.kind {
                ProtocolKind::Http => 4,
                ProtocolKind::Https => 5,
            }
        }
    }

    let https_protocol = Protocol {
        kind: ProtocolKind::Https,
    };
    
    assert_eq!(https_protocol.len(), 5);
}

