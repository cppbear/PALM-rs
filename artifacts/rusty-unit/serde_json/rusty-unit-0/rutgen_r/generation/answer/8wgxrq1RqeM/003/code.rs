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
fn test_serialize_positive_integer() {
    use serde_json::Serializer;
    
    let my_struct = MyStruct { n: N::PosInt(42) };
    let mut serializer = Serializer::new(Vec::new());
    
    let result = my_struct.serialize(&mut serializer).unwrap();
    
    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_serialize_large_positive_integer() {
    use serde_json::Serializer;
    
    let my_struct = MyStruct { n: N::PosInt(1_000_000_000) };
    let mut serializer = Serializer::new(Vec::new());
    
    let result = my_struct.serialize(&mut serializer).unwrap();
    
    assert_eq!(result.to_string(), "1000000000");
}

#[test]
fn test_serialize_zero_positive_integer() {
    use serde_json::Serializer;
    
    let my_struct = MyStruct { n: N::PosInt(0) };
    let mut serializer = Serializer::new(Vec::new());
    
    let result = my_struct.serialize(&mut serializer).unwrap();
    
    assert_eq!(result.to_string(), "0");
}

#[should_panic]
#[test]
fn test_serialize_negative_integer_should_panic() {
    use serde_json::Serializer;
    
    let my_struct = MyStruct { n: N::NegInt(-10) };
    let mut serializer = Serializer::new(Vec::new());
    
    my_struct.serialize(&mut serializer).unwrap();
}

#[should_panic]
#[test]
fn test_serialize_float_should_panic() {
    use serde_json::Serializer;
    
    let my_struct = MyStruct { n: N::Float(3.14) };
    let mut serializer = Serializer::new(Vec::new());
    
    my_struct.serialize(&mut serializer).unwrap();
}

