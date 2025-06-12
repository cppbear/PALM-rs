// Answer 0

#[test]
fn test_serialize_field_for_map() {
    use serde::ser::{Serialize, Serializer};
    use crate::ser::{Compound, SerializeMap};

    struct TestSerializer;

    impl Serializer for TestSerializer {
        // Required methods for Serializer implementation can go here
    }

    let mut map = SerializeMap::Map {
        map: Map::new(),
        next_key: None,
    };

    let mut compound = Compound::Map {
        ser: &mut TestSerializer,
        state: State::Empty,
    };

    assert_eq!(compound.serialize_field("test_key", &"test_value"), Ok(()));
}

#[test]
#[should_panic]
fn test_serialize_field_for_number_unreachable() {
    use crate::ser::{Compound};

    let mut compound = Compound::Number {
        ser: &mut TestSerializer,
    };

    compound.serialize_field("test_key", &"test_value");
}

#[test]
#[should_panic]
fn test_serialize_field_for_raw_value_unreachable() {
    use crate::ser::{Compound};

    let mut compound = Compound::RawValue {
        ser: &mut TestSerializer,
    };

    compound.serialize_field("test_key", &"test_value");
}

