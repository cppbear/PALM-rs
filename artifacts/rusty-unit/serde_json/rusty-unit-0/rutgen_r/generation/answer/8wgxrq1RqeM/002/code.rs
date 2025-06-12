// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

struct MyStruct {
    n: N,
}

impl MyStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.n {
            N::PosInt(u) => serializer.serialize_u64(u),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

#[test]
fn test_serialize_neg_int() {
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;

    let my_struct = MyStruct { n: N::NegInt(-42) };
    let mut serializer = Serializer::new(Vec::new());
    let result = my_struct.serialize(&mut serializer);
    
    assert!(result.is_ok());
}

#[test]
fn test_serialize_neg_int_min_value() {
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;

    let my_struct = MyStruct { n: N::NegInt(i64::MIN) };
    let mut serializer = Serializer::new(Vec::new());
    let result = my_struct.serialize(&mut serializer);
    
    assert!(result.is_ok());
}

#[test]
fn test_serialize_neg_int_max_value() {
    use serde_json::Serializer;
    use serde::ser::Serializer as SerdeSerializer;

    let my_struct = MyStruct { n: N::NegInt(-1) };
    let mut serializer = Serializer::new(Vec::new());
    let result = my_struct.serialize(&mut serializer);
    
    assert!(result.is_ok());
}

