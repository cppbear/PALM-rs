pub fn new(value: Cow<'a, str>) -> Self {
        CowStrDeserializer {
            value,
            marker: PhantomData,
        }
    }