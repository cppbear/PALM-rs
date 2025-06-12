// Answer 0

#[derive(Debug)]
struct Decoder {
    conservative_decoded_len: usize,
}

impl Decoder {
    fn decoded_len_estimate(&self) -> usize {
        self.conservative_decoded_len
    }
}

#[test]
fn test_decoded_len_estimate() {
    let decoder = Decoder {
        conservative_decoded_len: 10,
    };
    assert_eq!(decoder.decoded_len_estimate(), 10);
}

#[test]
fn test_decoded_len_estimate_zero() {
    let decoder = Decoder {
        conservative_decoded_len: 0,
    };
    assert_eq!(decoder.decoded_len_estimate(), 0);
}

#[test]
fn test_decoded_len_estimate_large() {
    let decoder = Decoder {
        conservative_decoded_len: usize::MAX,
    };
    assert_eq!(decoder.decoded_len_estimate(), usize::MAX);
}

