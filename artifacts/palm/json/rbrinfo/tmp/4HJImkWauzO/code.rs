fn new(map: Map<String, Value>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }