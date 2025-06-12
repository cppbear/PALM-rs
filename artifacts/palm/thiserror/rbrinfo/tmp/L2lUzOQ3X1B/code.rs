fn provide<'a>(&'a self, request: &mut Request<'a>) {
        request.provide_ref(&self.0);
    }