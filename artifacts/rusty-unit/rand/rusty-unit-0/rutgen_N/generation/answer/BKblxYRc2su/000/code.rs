// Answer 0

#[test]
fn test_set_nonce() {
    struct NonceSetter {
        nonce: u64,
    }

    fn set_stream_param(setter: &mut NonceSetter, param: u32, value: u64) {
        setter.nonce = value;
    }

    const STREAM_PARAM_NONCE: u32 = 1;

    let mut setter = NonceSetter { nonce: 0 };
    setter.set_nonce(42);
    
    assert_eq!(setter.nonce, 42);
}

