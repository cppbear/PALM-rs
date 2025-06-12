pub fn new(body: T) -> Response<T> {
        Response {
            head: Parts::new(),
            body,
        }
    }