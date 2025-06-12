// Answer 0

#[test]
fn test_into_mut_valid() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 100);
    let value: &mut u32;

    match map.raw_entry_mut().from_key(&1) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => value = o.into_mut(),
    }
    *value += 900;
}

#[test]
#[should_panic]
fn test_into_mut_key_not_found() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(2, 200);
    let value: &mut u32;

    match map.raw_entry_mut().from_key(&1) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => value = o.into_mut(),
    }
}

#[test]
fn test_into_mut_multiple_entries() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(3, 300);
    map.insert(4, 400);
    let value: &mut u32;

    match map.raw_entry_mut().from_key(&3) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => value = o.into_mut(),
    }
    *value += 750;
}

#[test]
fn test_into_mut_large_value() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(5, 9999);
    let value: &mut u32;

    match map.raw_entry_mut().from_key(&5) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => value = o.into_mut(),
    }
    *value += 1;
}

#[test]
fn test_into_mut_high_keys() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1000, 10000);
    let value: &mut u32;

    match map.raw_entry_mut().from_key(&1000) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => value = o.into_mut(),
    }
    *value += 50;
}

