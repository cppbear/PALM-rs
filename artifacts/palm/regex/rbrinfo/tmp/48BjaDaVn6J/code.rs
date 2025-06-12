fn find_cap_ref<T: ?Sized + AsRef<[u8]>>(
    replacement: &T,
) -> Option<CaptureRef> {
    let mut i = 0;
    let rep: &[u8] = replacement.as_ref();
    if rep.len() <= 1 || rep[0] != b'$' {
        return None;
    }
    let mut brace = false;
    i += 1;
    if rep[i] == b'{' {
        brace = true;
        i += 1;
    }
    let mut cap_end = i;
    while rep.get(cap_end).map_or(false, is_valid_cap_letter) {
        cap_end += 1;
    }
    if cap_end == i {
        return None;
    }
    // We just verified that the range 0..cap_end is valid ASCII, so it must
    // therefore be valid UTF-8. If we really cared, we could avoid this UTF-8
    // check with either unsafe or by parsing the number straight from &[u8].
    let cap = str::from_utf8(&rep[i..cap_end])
                  .expect("valid UTF-8 capture name");
    if brace {
        if !rep.get(cap_end).map_or(false, |&b| b == b'}') {
            return None;
        }
        cap_end += 1;
    }
    Some(CaptureRef {
        cap: match cap.parse::<u32>() {
            Ok(i) => Ref::Number(i as usize),
            Err(_) => Ref::Named(cap),
        },
        end: cap_end,
    })
}