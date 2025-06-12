// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestKey(&'static str);

    #[derive(Debug, Clone, PartialEq)]
    struct TestValue(i32);

    #[test]
    fn test_from_key_when_key_exists() {
        let mut map = IndexMap::new();
        map.insert(TestKey("existing"), TestValue(42));

        let builder = RawEntryBuilderMut { map: &mut map };
        match builder.from_key(&TestKey("existing")) {
            RawEntryMut::Occupied(occupied) => {
                // Here we could add assertions regarding the occupied entry
                // For example, we might fetch the value and check it.
            }
            RawEntryMut::Vacant(_) => panic!("Expected occupied entry, but found vacant."),
        }
    }

    #[test]
    fn test_from_key_when_key_does_not_exist() {
        let mut map = IndexMap::new();
        let builder = RawEntryBuilderMut { map: &mut map };
        match builder.from_key(&TestKey("non_existing")) {
            RawEntryMut::Vacant(_) => {
                // This is expected, no entry should exist
            }
            RawEntryMut::Occupied(_) => panic!("Expected vacant entry, but found occupied."),
        }
    }

    #[test]
    fn test_from_key_with_different_key_type() {
        let mut map = IndexMap::new();
        map.insert(TestKey("number"), TestValue(100));

        let builder = RawEntryBuilderMut { map: &mut map };
        match builder.from_key(&TestKey("number")) {
            RawEntryMut::Occupied(_) => {
                // The entry "number" exists, so we expect it to be occupied
            }
            RawEntryMut::Vacant(_) => panic!("Expected occupied entry, but found vacant."),
        }

        match builder.from_key(&TestKey("not_present")) {
            RawEntryMut::Vacant(_) => {
                // This is expected since "not_present" does not exist in map
            }
            RawEntryMut::Occupied(_) => panic!("Expected vacant entry, but found occupied."),
        }
    }
}

