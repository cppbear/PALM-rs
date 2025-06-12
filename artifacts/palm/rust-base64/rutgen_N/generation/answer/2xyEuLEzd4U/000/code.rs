// Answer 0

#[derive(Default)]
struct SimpleEngine {
    padding: bool,
}

impl SimpleEngine {
    fn config(&self) -> SimpleConfig {
        SimpleConfig { padding: self.padding }
    }

    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
        let encoded = base64::encode(input);
        let bytes_written = encoded.as_bytes().iter().copied().take(output.len()).collect::<Vec<_>>();
        output[..bytes_written.len()].copy_from_slice(&bytes_written);
        bytes_written.len()
    }
}

struct SimpleConfig {
    padding: bool,
}

impl SimpleConfig {
    fn encode_padding(&self) -> bool {
        self.padding
    }
}

fn add_padding(bytes_written: usize, padding: &mut [u8]) -> usize {
    let padding_len = 4 - (bytes_written % 4);
    if padding.len() < padding_len {
        return 0;
    }
    padding[..padding_len].fill(b'=');
    padding_len
}

#[test]
fn test_encode_with_padding() {
    let input = b"hello";
    let expected_size = 8; // Base64 of "hello" is 5 bytes + 3 = 8 with padding
    let mut output = vec![0u8; expected_size];
    let engine = SimpleEngine { padding: true };

    encode_with_padding(input, &mut output, &engine, expected_size);

    assert_eq!(output, b"aGVsbG8=");
}

#[test]
fn test_encode_without_padding() {
    let input = b"hello";
    let expected_size = 8; // Base64 of "hello" is 5 bytes + 3 = 8 with padding
    let mut output = vec![0u8; expected_size];
    let engine = SimpleEngine { padding: false };

    encode_with_padding(input, &mut output, &engine, expected_size);

    assert_eq!(output, b"aGVsbG8=");
}

#[test]
#[should_panic]
fn test_encode_with_incorrect_size() {
    let input = b"hello";
    let expected_size = 9; // Intentional wrong size
    let mut output = vec![0u8; 8]; // Incorrect output size
    let engine = SimpleEngine { padding: true };

    encode_with_padding(input, &mut output, &engine, expected_size);
}

