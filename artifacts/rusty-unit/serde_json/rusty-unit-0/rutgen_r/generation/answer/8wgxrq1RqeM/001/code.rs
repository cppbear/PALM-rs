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

trait Serializer {
    type Ok;
    type Error;

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error>;
}

struct MockSerializer {
    output: Option<f64>,
}

impl Serializer for MockSerializer {
    type Ok = ();
    type Error = ();

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        panic!("serialize_u64 should not be called");
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        panic!("serialize_i64 should not be called");
    }
    
    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.output = Some(value);
        Ok(())
    }
}

impl MyStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(u) => serializer.serialize_u64(u),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

#[test]
fn test_serialize_float() {
    let my_struct = MyStruct { n: N::Float(3.14) };
    let mut serializer = MockSerializer { output: None };
    
    let result = my_struct.serialize(serializer);
    
    assert!(result.is_ok());
    assert_eq!(serializer.output, Some(3.14));
}

#[test]
fn test_serialize_negative_float() {
    let my_struct = MyStruct { n: N::Float(-1.23) };
    let mut serializer = MockSerializer { output: None };
    
    let result = my_struct.serialize(serializer);
    
    assert!(result.is_ok());
    assert_eq!(serializer.output, Some(-1.23));
}

#[test]
fn test_serialize_zero_float() {
    let my_struct = MyStruct { n: N::Float(0.0) };
    let mut serializer = MockSerializer { output: None };
    
    let result = my_struct.serialize(serializer);
    
    assert!(result.is_ok());
    assert_eq!(serializer.output, Some(0.0));
}

