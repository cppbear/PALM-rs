pub fn new(content: Content<'de>) -> Self {
            ContentDeserializer {
                content,
                err: PhantomData,
            }
        }