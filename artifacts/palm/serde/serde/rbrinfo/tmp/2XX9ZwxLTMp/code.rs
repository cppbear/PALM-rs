pub fn new(variant: Content<'de>, value: Option<Content<'de>>) -> EnumDeserializer<'de, E> {
            EnumDeserializer {
                variant,
                value,
                err: PhantomData,
            }
        }