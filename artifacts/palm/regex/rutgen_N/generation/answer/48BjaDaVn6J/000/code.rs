// Answer 0

#[test]
fn test_find_cap_ref_valid_named_capture() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }

    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(b: &u8) -> bool {
        (*b >= b'a' && *b <= b'z') || (*b >= b'A' && *b <= b'Z') || (*b == b'_' || (*b >= b'0' && *b <= b'9'))
    }

    fn find_cap_ref<T: ?Sized + AsRef<[u8]>>(replacement: &T) -> Option<CaptureRef> {
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
        let cap = std::str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
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

    let result = find_cap_ref(b"${name}");
    assert!(result.is_some());
    let capture = result.unwrap();
    if let Ref::Named(name) = capture.cap {
        assert_eq!(name, "name");
    } else {
        panic!("Expected a named reference.");
    }
    assert_eq!(capture.end, 6);
}

#[test]
fn test_find_cap_ref_valid_numeric_capture() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }

    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(b: &u8) -> bool {
        (*b >= b'a' && *b <= b'z') || (*b >= b'A' && *b <= b'Z') || (*b == b'_' || (*b >= b'0' && *b <= b'9'))
    }

    fn find_cap_ref<T: ?Sized + AsRef<[u8]>>(replacement: &T) -> Option<CaptureRef> {
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
        let cap = std::str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
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

    let result = find_cap_ref(b"$12");
    assert!(result.is_some());
    let capture = result.unwrap();
    if let Ref::Number(num) = capture.cap {
        assert_eq!(num, 12);
    } else {
        panic!("Expected a numeric reference.");
    }
    assert_eq!(capture.end, 3);
}

#[test]
fn test_find_cap_ref_invalid_capture() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }

    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(b: &u8) -> bool {
        (*b >= b'a' && *b <= b'z') || (*b >= b'A' && *b <= b'Z') || (*b == b'_' || (*b >= b'0' && *b <= b'9'))
    }

    fn find_cap_ref<T: ?Sized + AsRef<[u8]>>(replacement: &T) -> Option<CaptureRef> {
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
        let cap = std::str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
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

    let result = find_cap_ref(b"$");
    assert!(result.is_none());

    let result_brace = find_cap_ref(b"${");
    assert!(result_brace.is_none());

    let result_no_closing_brace = find_cap_ref(b"${name");
    assert!(result_no_closing_brace.is_none());
}

