// Answer 0

#[test]
fn test_size_hint_none_case() {
    let lower = 0;
    let upper = 1;
    let iter = IntoIter {
        iter: IntoIterImpl {
            size_hint: (lower, Some(upper)),
        },
    };
    let map_deserializer = MapDeserializer {
        iter,
        value: None,
    };
    map_deserializer.size_hint();
}

#[test]
fn test_size_hint_non_matching_case() {
    let lower = 2;
    let upper = 4;
    let iter = IntoIter {
        iter: IntoIterImpl {
            size_hint: (lower, Some(upper)),
        },
    };
    let map_deserializer = MapDeserializer {
        iter,
        value: None,
    };
    map_deserializer.size_hint();
}

#[test]
fn test_size_hint_empty_case() {
    let lower = 0;
    let upper = 0;
    let iter = IntoIter {
        iter: IntoIterImpl {
            size_hint: (lower, Some(upper)),
        },
    };
    let map_deserializer = MapDeserializer {
        iter,
        value: None,
    };
    map_deserializer.size_hint();
}

