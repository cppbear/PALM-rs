fn new(map: &'de Map<String, Value>) -> Self {
        MapRefDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }