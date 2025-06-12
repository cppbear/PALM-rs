pub fn body<T>(self, body: T) -> Result<Request<T>> {
        self.inner.map(move |head| Request { head, body })
    }