pub fn new<T>(inner: T, limit: usize) -> Take<T> {
    Take { inner, limit }
}