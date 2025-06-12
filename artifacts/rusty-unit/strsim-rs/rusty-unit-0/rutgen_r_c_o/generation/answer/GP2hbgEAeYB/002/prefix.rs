// Answer 0

#[test]
fn test_get_hybrid_growing_hashmap_char_above_extended_ascii() {
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: Some(vec![]),
        },
        extended_ascii: [0; 256],
    };

    let result1 = hashmap.get('\u{0100}');
    let result2 = hashmap.get('\u{7FFF}');
    let result3 = hashmap.get('\u{FFFF}');
}

#[test]
fn test_get_hybrid_growing_hashmap_char_high_unicode() {
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 1,
            fill: 1,
            mask: 1,
            map: Some(vec![GrowingHashmapMapElemChar { value: 42 }]),
        },
        extended_ascii: [0; 256],
    };

    hashmap.map.allocate();

    let result1 = hashmap.get('\u{0101}');
    let result2 = hashmap.get('\u{1234}');
    let result3 = hashmap.get('\u{FFFE}');
}

