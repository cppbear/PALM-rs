pub(super) fn new<T>(inner: T, limit: usize) -> Limit<T> {
    Limit { inner, limit }
}