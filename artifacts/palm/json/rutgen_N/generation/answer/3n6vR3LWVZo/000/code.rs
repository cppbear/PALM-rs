// Answer 0

#[test]
fn test_begin_object_value() {
    use std::io::Cursor;
    use std::io::Write;
    use std::io;

    struct TestStruct;

    impl TestStruct {
        fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            writer.write_all(b": ")
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut test_struct = TestStruct;

    let result = test_struct.begin_object_value(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.into_inner(), b": ".to_vec());
}

#[test]
fn test_begin_object_value_empty_writer() {
    use std::io::Cursor;
    use std::io::Write;
    use std::io;

    struct TestStruct;

    impl TestStruct {
        fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            writer.write_all(b": ")
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut test_struct = TestStruct;

    let result = test_struct.begin_object_value(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.into_inner(), b": ".to_vec());
}

