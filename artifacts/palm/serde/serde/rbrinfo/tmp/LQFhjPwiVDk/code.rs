fn clone(&self) -> Self {
        StringDeserializer {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }