pub fn new(content: &'a Content<'de>) -> Self {
            ContentRefDeserializer {
                content,
                err: PhantomData,
            }
        }