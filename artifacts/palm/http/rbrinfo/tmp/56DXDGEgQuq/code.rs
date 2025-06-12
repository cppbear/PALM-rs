fn from(port: Port<T>) -> Self {
        port.as_u16()
    }