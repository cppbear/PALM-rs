pub fn new(body: T) -> Request<T> {
        Request {
            head: Parts::new(),
            body,
        }
    }