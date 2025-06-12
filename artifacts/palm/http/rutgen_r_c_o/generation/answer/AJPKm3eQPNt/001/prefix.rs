// Answer 0

#[test]
fn test_entry_with_default_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(0);
    let _ = map.entry("header-1");
    let _ = map.entry("header-2");
}

#[test]
fn test_entry_with_small_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(5);
    let _ = map.entry("header-a");
    let _ = map.entry("header-b");
}

#[test]
fn test_entry_with_large_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(32768);
    let _ = map.entry("header-max-capacity");
    let _ = map.entry("header-another");
}

#[should_panic]
fn test_entry_exceed_max_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap::try_with_capacity(32769).unwrap_err();
    let _ = map.entry("overflow-header");
}

#[test]
fn test_entry_with_case_insensitive_keys() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    let _ = map.entry("Content-Type");
    let _ = map.entry("content-type");
}

#[test]
fn test_entry_with_non_ascii_keys() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    let _ = map.entry("header-äüö");
    let _ = map.entry("header-ß");
}

#[test]
fn test_entry_with_empty_key() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    let _ = map.entry("");
}

