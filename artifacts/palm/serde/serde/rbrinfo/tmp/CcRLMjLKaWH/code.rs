fn flat_map_take_entry<'de>(
    entry: &mut Option<(Content<'de>, Content<'de>)>,
    recognized: &[&str],
) -> Option<(Content<'de>, Content<'de>)> {
    // Entries in the FlatMapDeserializer buffer are nulled out as they get
    // claimed for deserialization. We only use an entry if it is still present
    // and if the field is one recognized by the current data structure.
    let is_recognized = match entry {
        None => false,
        Some((k, _v)) => k.as_str().map_or(false, |name| recognized.contains(&name)),
    };

    if is_recognized {
        entry.take()
    } else {
        None
    }
}