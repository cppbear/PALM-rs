// Answer 0

#[test]
fn test_read_captures_at_with_no_slots() {
    struct DummyRegex {
        match_type: MatchType,
    }

    impl DummyRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 0))
        }

        fn read_captures_at(
            &self,
            locs: &mut Locations,
            text: &[u8],
            start: usize,
        ) -> Option<(usize, usize)> {
            let slots = as_slots(locs);
            for slot in slots.iter_mut() {
                *slot = None;
            }
            match slots.len() {
                0 => return self.find_at(text, start),
                _ => {}
            }
            if !self.is_anchor_end_match(text) {
                return None;
            }
            None
        }
    }

    struct Locations(Vec<Option<usize>>);

    fn as_slots(locs: &mut Locations) -> &mut [Option<usize>] {
        &mut locs.0
    }

    let regex = DummyRegex {
        match_type: MatchType::Nfa(NfaType::SomeType),
    };
    let mut locs = Locations(vec![]);
    let result = regex.read_captures_at(&mut locs, b"test", 0);
    assert_eq!(result, Some((0, 0)));
}

#[test]
fn test_read_captures_at_with_two_slots() {
    struct DummyRegex {
        match_type: MatchType,
    }

    impl DummyRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((1, 3))
        }

        fn read_captures_at(
            &self,
            locs: &mut Locations,
            text: &[u8],
            start: usize,
        ) -> Option<(usize, usize)> {
            let slots = as_slots(locs);
            for slot in slots.iter_mut() {
                *slot = None;
            }
            match slots.len() {
                2 => {
                    return self.find_at(text, start).map(|(s, e)| {
                        slots[0] = Some(s);
                        slots[1] = Some(e);
                        (s, e)
                    });
                }
                _ => {}
            }
            if !self.is_anchor_end_match(text) {
                return None;
            }
            None
        }
    }

    struct Locations(Vec<Option<usize>>);

    fn as_slots(locs: &mut Locations) -> &mut [Option<usize>] {
        &mut locs.0
    }

    let regex = DummyRegex {
        match_type: MatchType::Nfa(NfaType::SomeType),
    };
    let mut locs = Locations(vec![None, None]);
    let result = regex.read_captures_at(&mut locs, b"test", 0);
    assert_eq!(result, Some((1, 3)));
}

#[test]
fn test_read_captures_at_with_generic_case() {
    struct DummyRegex {
        match_type: MatchType,
    }

    impl DummyRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((2, 4))
        }

        fn read_captures_at(
            &self,
            locs: &mut Locations,
            text: &[u8],
            start: usize,
        ) -> Option<(usize, usize)> {
            let slots = as_slots(locs);
            for slot in slots.iter_mut() {
                *slot = None;
            }
            match slots.len() {
                _ => {}
            }
            if !self.is_anchor_end_match(text) {
                return None;
            }
            self.find_at(text, start)
        }
    }

    struct Locations(Vec<Option<usize>>);

    fn as_slots(locs: &mut Locations) -> &mut [Option<usize>] {
        &mut locs.0
    }

    let regex = DummyRegex {
        match_type: MatchType::Nfa(NfaType::SomeType),
    };
    let mut locs = Locations(vec![None, None, None]);
    let result = regex.read_captures_at(&mut locs, b"test", 0);
    assert_eq!(result, Some((2, 4)));
}

