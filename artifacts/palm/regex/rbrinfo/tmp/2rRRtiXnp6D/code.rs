pub fn expand_bytes(
    caps: &re_bytes::Captures,
    mut replacement: &[u8],
    dst: &mut Vec<u8>,
) {
    while !replacement.is_empty() {
        match memchr(b'$', replacement) {
            None => break,
            Some(i) => {
                dst.extend(&replacement[..i]);
                replacement = &replacement[i..];
            }
        }
        if replacement.get(1).map_or(false, |&b| b == b'$') {
            dst.push(b'$');
            replacement = &replacement[2..];
            continue;
        }
        debug_assert!(!replacement.is_empty());
        let cap_ref = match find_cap_ref(replacement) {
            Some(cap_ref) => cap_ref,
            None => {
                dst.push(b'$');
                replacement = &replacement[1..];
                continue;
            }
        };
        replacement = &replacement[cap_ref.end..];
        match cap_ref.cap {
            Ref::Number(i) => {
                dst.extend(
                    caps.get(i).map(|m| m.as_bytes()).unwrap_or(b""));
            }
            Ref::Named(name) => {
                dst.extend(
                    caps.name(name).map(|m| m.as_bytes()).unwrap_or(b""));
            }
        }
    }
    dst.extend(replacement);
}