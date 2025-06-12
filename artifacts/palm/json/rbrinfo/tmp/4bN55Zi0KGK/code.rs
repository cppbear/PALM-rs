fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }