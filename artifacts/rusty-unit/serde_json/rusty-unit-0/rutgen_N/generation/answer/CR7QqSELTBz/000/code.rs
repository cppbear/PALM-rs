// Answer 0

#[test]
fn test_into_inner() {
    struct MockWriter;
    
    struct Serializer<W> {
        writer: W,
    }

    let writer = MockWriter;
    let serializer = Serializer { writer };

    let unwrapped_writer = serializer.into_inner();
    assert!(std::mem::size_of_val(&unwrapped_writer) == std::mem::size_of::<MockWriter>());
}

