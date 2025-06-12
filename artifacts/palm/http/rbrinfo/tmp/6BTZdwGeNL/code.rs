pub fn body<T>(self, body: T) -> Result<Response<T>> {
        self.inner.map(move |head| Response { head, body })
    }