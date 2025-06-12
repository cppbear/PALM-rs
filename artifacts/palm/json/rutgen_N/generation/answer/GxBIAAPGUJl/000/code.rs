// Answer 0

#[test]
fn test_begin_array() {
    use std::io::{self, Cursor};

    struct Serializer {
        current_indent: usize,
        has_value: bool,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                current_indent: 0,
                has_value: false,
            }
        }

        fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()> 
        where
            W: ?Sized + io::Write,
        {
            self.current_indent += 1;
            self.has_value = false;
            writer.write_all(b"[")
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut serializer = Serializer::new();
    
    let result = serializer.begin_array(&mut writer);
    
    assert!(result.is_ok());
    assert_eq!(writer.get_ref(), b"[");
    assert_eq!(serializer.current_indent, 1);
    assert!(!serializer.has_value);
}

