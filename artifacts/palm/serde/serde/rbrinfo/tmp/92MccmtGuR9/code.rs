fn clone(&self) -> Self {
        CowStrDeserializer {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }