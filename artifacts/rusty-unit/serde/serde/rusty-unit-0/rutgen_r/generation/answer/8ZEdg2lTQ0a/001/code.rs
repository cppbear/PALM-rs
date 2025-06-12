// Answer 0

#[derive(Debug)]
struct MySerializer;

impl MySerializer {
    fn bad_type(_: Unsupported) -> String {
        "Unsupported type".to_string()
    }
    
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<String, String> {
        Err(Self::bad_type(Unsupported::TupleStruct))
    }
}

#[derive(Debug)]
enum Unsupported {
    TupleStruct,
}

#[test]
fn test_serialize_tuple_struct_returns_err_for_tuple_struct() {
    let serializer = MySerializer;
    let result = serializer.serialize_tuple_struct("MyTupleStruct", 2);
    assert_eq!(result, Err("Unsupported type".to_string()));
}

