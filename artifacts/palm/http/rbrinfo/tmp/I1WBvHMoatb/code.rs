fn drain_all_extra_values<T>(
    raw_links: RawLinks<T>,
    extra_values: &mut Vec<ExtraValue<T>>,
    mut head: usize,
) -> Vec<T> {
    let mut vec = Vec::new();
    loop {
        let extra = remove_extra_value(raw_links, extra_values, head);
        vec.push(extra.value);

        if let Link::Extra(idx) = extra.next {
            head = idx;
        } else {
            break;
        }
    }
    vec
}