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
    use serde::ser::Serialize;

    let my_struct = MyStruct { n: N::PosInt(42) };
    let mut vec = Vec::new();
    let serializer = Serializer::new(&mut vec);
    let result = my_struct.serialize(serializer);

    assert!(result.is_ok());
    let value: u64 = serde_json::from_slice(&vec).unwrap();
    assert_eq!(value, 42);
}

#[test]
fn test_serialize_large_positive_integer() {
    use serde_json::Serializer;
    use serde::ser::Serialize;

    let my_struct = MyStruct { n: N::PosInt(u64::MAX) };
    let mut vec = Vec::new();
    let serializer = Serializer::new(&mut vec);
    let result = my_struct.serialize(serializer);

    assert!(result.is_ok());
    let value: u64 = serde_json::from_slice(&vec).unwrap();
    assert_eq!(value, u64::MAX);
}

