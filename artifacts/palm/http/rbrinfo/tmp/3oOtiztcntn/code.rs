fn append_value<T>(
    entry_idx: usize,
    entry: &mut Bucket<T>,
    extra: &mut Vec<ExtraValue<T>>,
    value: T,
) {
    match entry.links {
        Some(links) => {
            let idx = extra.len();
            extra.push(ExtraValue {
                value,
                prev: Link::Extra(links.tail),
                next: Link::Entry(entry_idx),
            });

            extra[links.tail].next = Link::Extra(idx);

            entry.links = Some(Links { tail: idx, ..links });
        }
        None => {
            let idx = extra.len();
            extra.push(ExtraValue {
                value,
                prev: Link::Entry(entry_idx),
                next: Link::Entry(entry_idx),
            });

            entry.links = Some(Links {
                next: idx,
                tail: idx,
            });
        }
    }
}