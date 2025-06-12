fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        de::Error::custom(msg)
    }