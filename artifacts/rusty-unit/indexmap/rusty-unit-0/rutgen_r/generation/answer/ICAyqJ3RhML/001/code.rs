// Answer 0

fn test_fmt_vacant() {
    struct VacantEntry;

    enum Entry {
        Vacant(VacantEntry),
        Occupied,
    }

    impl std::fmt::Debug for VacantEntry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("VacantEntry")
        }
    }

    impl Entry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut tuple = f.debug_tuple("Entry");
            match self {
                Entry::Vacant(v) => tuple.field(v),
                Entry::Occupied => tuple.field(&"Occupied"),
            };
            tuple.finish()
        }
    }

    let vacant_entry = Entry::Vacant(VacantEntry);
    let result = format!("{:?}", vacant_entry);
    assert_eq!(result, "Entry(VacantEntry)");
}

fn test_fmt_occupied() {
    struct OccupiedEntry;

    enum Entry {
        Vacant,
        Occupied(OccupiedEntry),
    }

    impl std::fmt::Debug for OccupiedEntry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("OccupiedEntry")
        }
    }

    impl Entry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut tuple = f.debug_tuple("Entry");
            match self {
                Entry::Vacant => tuple.field(&"Vacant"),
                Entry::Occupied(o) => tuple.field(o),
            };
            tuple.finish()
        }
    }

    let occupied_entry = Entry::Occupied(OccupiedEntry);
    let result = format!("{:?}", occupied_entry);
    assert_eq!(result, "Entry(OccupiedEntry)");
}

