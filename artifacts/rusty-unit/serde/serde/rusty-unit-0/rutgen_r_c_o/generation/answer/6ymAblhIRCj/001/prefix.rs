// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let mut map = MySerializeMap {};
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_one() {
    let mut map = MySerializeMap {};
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_max() {
    let mut map = MySerializeMap {};
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u8(254);
}

#[test]
fn test_serialize_u8255() {
    let mut map = MySerializeMap {};
    let serializer = FlatMapSerializer(&mut map);
    let _ = serializer.serialize_u8(255);
}

struct MySerializeMap;

impl SerializeMap for MySerializeMap {
    type Ok = ();
    type Error = Error;
    
    fn serialize_key<K>(&mut self, _key: K) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, _key: K, _value: &V) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

