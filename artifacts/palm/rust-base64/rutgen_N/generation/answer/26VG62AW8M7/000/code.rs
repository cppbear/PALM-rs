// Answer 0

#[test]
fn test_base64_display_new() {
    struct MockEngine;

    struct Base64Display<'a, 'e, E> {
        bytes: &'a [u8],
        chunked_encoder: ChunkedEncoder<'e, E>,
    }

    struct ChunkedEncoder<'e, E> {
        engine: &'e E,
    }

    impl<'e, E> ChunkedEncoder<'e, E> {
        fn new(engine: &'e E) -> Self {
            ChunkedEncoder { engine }
        }
    }

    let bytes = b"Hello, world!";
    let engine = MockEngine;

    let display = Base64Display::new(bytes, &engine);

    assert_eq!(display.bytes, bytes);
}

