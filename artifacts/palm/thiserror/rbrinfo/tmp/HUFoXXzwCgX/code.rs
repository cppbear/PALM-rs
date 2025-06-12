fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>) {
        self.provide(request);
    }