// Answer 0

#[test]
fn test_protocol_https_len() {
    struct TestProtocol {
        protocol: super::Protocol,
    }

    let test_instance = TestProtocol {
        protocol: super::Protocol::Https,
    };

    assert_eq!(test_instance.protocol.len(), 5);
}

