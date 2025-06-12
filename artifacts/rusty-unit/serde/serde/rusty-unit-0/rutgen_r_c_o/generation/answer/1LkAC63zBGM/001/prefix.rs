// Answer 0

#[test]
fn test_serialize_char_valid_basic() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('a');
}

#[test]
fn test_serialize_char_valid_control() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('\u{0000}');
}

#[test]
fn test_serialize_char_valid_non_printable() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('\u{007F}');
}

#[test]
fn test_serialize_char_valid_surrogate_low() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('\u{E000}');
}

#[test]
fn test_serialize_char_valid_surrogate_high() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('\u{FFFF}');
}

#[test]
fn test_serialize_char_valid_extended() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('\u{10000}');
}

#[test]
fn test_serialize_char_valid_max() {
    let serializer = ContentSerializer { error: PhantomData };
    let _ = serializer.serialize_char('\u{10FFFF}');
}

