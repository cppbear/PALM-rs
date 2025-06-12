fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            err: msg.to_string().into_boxed_str(),
        }
    }