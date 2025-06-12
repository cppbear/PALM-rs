pub fn get_span<T>(span: T) -> GetSpan<T> {
    GetSpan(GetSpanInner(GetSpanBase(span)))
}