fn from(status: StatusCode) -> u16 {
        status.0.get()
    }