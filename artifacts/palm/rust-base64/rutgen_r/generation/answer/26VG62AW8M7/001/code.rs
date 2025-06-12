// Answer 0

#[test]
fn test_new_base64_display() {
    struct MockEngine;

    struct ChunkedEncoder<'e> {
        engine: &'e MockEngine,
    }

    impl<'e> ChunkedEncoder<'e> {
        fn new(engine: &'e MockEngine) -> Self {
            ChunkedEncoder { engine }
        }
    }

    struct Base64Display<'a, 'e, E> {
        bytes: &'a [u8],
        chunked_encoder: ChunkedEncoder<'e>,
    }

    let bytes: &[u8] = b"Hello, world!";
    let engine = MockEngine;

    let display = new(bytes, &engine);
    assert_eq!(display.bytes, bytes);
}

#[test]
#[should_panic]
fn test_new_base64_display_empty_bytes() {
    struct MockEngine;

    struct ChunkedEncoder<'e> {
        engine: &'e MockEngine,
    }

    impl<'e> ChunkedEncoder<'e> {
        fn new(engine: &'e MockEngine) -> Self {
            ChunkedEncoder { engine }
        }
    }

    struct Base64Display<'a, 'e, E> {
        bytes: &'a [u8],
        chunked_encoder: ChunkedEncoder<'e>,
    }

    let bytes: &[u8] = &[];
    let engine = MockEngine;

    let _display = new(bytes, &engine);
}

#[test]
fn test_new_base64_display_boundary() {
    struct MockEngine;

    struct ChunkedEncoder<'e> {
        engine: &'e MockEngine,
    }

    impl<'e> ChunkedEncoder<'e> {
        fn new(engine: &'e MockEngine) -> Self {
            ChunkedEncoder { engine }
        }
    }

    struct Base64Display<'a, 'e, E> {
        bytes: &'a [u8],
        chunked_encoder: ChunkedEncoder<'e>,
    }

    let bytes: &[u8] = b"";
    let engine = MockEngine;

    let display = new(bytes, &engine);
    assert_eq!(display.bytes.len(), 0);
}

