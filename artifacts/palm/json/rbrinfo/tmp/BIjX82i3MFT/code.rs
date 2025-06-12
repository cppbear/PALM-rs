pub fn into_iter<T>(self) -> StreamDeserializer<'de, R, T>
    where
        T: de::Deserialize<'de>,
    {
        // This cannot be an implementation of std::iter::IntoIterator because
        // we need the caller to choose what T is.
        let offset = self.read.byte_offset();
        StreamDeserializer {
            de: self,
            offset,
            failed: false,
            output: PhantomData,
            lifetime: PhantomData,
        }
    }