// Answer 0

#[test]
fn test_c_repeat_one_or_more_expr_err() {
    struct MockSelf;

    impl MockSelf {
        fn c(&mut self, _expr: &Hir) -> Result {
            Err("Error")
        }

        fn fill_to_next(&mut self, _hole_rep: usize) {}

        fn push_split_hole(&mut self) -> usize {
            0
        }

        fn fill_split(&mut self, _split: usize, _entry_rep: Option<usize>, _none: Option<usize>) -> usize {
            0
        }

        fn c_repeat_one_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {
            let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;
            self.fill_to_next(hole_rep);
            let split = self.push_split_hole();

            let split_hole = if greedy {
                self.fill_split(split, Some(entry_rep), None)
            } else {
                self.fill_split(split, None, Some(entry_rep))
            };
            Ok(Patch { hole: split_hole, entry: entry_rep })
        }
    }

    struct Hir;
    struct Patch {
        hole: usize,
        entry: usize,
    }

    let mut instance = MockSelf;
    let expr = Hir;

    // This line should panic due to the self.c(expr)? returning an Err
    let result = instance.c_repeat_one_or_more(&expr, true);
    assert!(result.is_err());
}

#[test]
fn test_c_repeat_one_or_more_greedy() {
    struct MockSelf {
        expected_hole: usize,
        expected_entry: usize,
    }

    impl MockSelf {
        fn c(&mut self, _expr: &Hir) -> Result {
            Ok(Patch { hole: self.expected_hole, entry: self.expected_entry })
        }

        fn fill_to_next(&mut self, _hole_rep: usize) {}

        fn push_split_hole(&mut self) -> usize {
            0
        }

        fn fill_split(&mut self, _split: usize, entry_rep: Option<usize>, _none: Option<usize>) -> usize {
            entry_rep.unwrap_or(0)
        }

        fn c_repeat_one_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {
            let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;
            self.fill_to_next(hole_rep);
            let split = self.push_split_hole();

            let split_hole = if greedy {
                self.fill_split(split, Some(entry_rep), None)
            } else {
                self.fill_split(split, None, Some(entry_rep))
            };
            Ok(Patch { hole: split_hole, entry: entry_rep })
        }
    }

    struct Hir;
    struct Patch {
        hole: usize,
        entry: usize,
    }

    let mut instance = MockSelf { expected_hole: 5, expected_entry: 10 };
    let expr = Hir;

    let result = instance.c_repeat_one_or_more(&expr, true).unwrap();
    assert_eq!(result.hole, 10);
    assert_eq!(result.entry, 10);
}

#[test]
fn test_c_repeat_one_or_more_non_greedy() {
    struct MockSelf {
        expected_hole: usize,
        expected_entry: usize,
    }

    impl MockSelf {
        fn c(&mut self, _expr: &Hir) -> Result {
            Ok(Patch { hole: self.expected_hole, entry: self.expected_entry })
        }

        fn fill_to_next(&mut self, _hole_rep: usize) {}

        fn push_split_hole(&mut self) -> usize {
            0
        }

        fn fill_split(&mut self, _split: usize, _none: Option<usize>, entry_rep: Option<usize>) -> usize {
            entry_rep.unwrap_or(0)
        }

        fn c_repeat_one_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {
            let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;
            self.fill_to_next(hole_rep);
            let split = self.push_split_hole();

            let split_hole = if greedy {
                self.fill_split(split, Some(entry_rep), None)
            } else {
                self.fill_split(split, None, Some(entry_rep))
            };
            Ok(Patch { hole: split_hole, entry: entry_rep })
        }
    }

    struct Hir;
    struct Patch {
        hole: usize,
        entry: usize,
    }

    let mut instance = MockSelf { expected_hole: 5, expected_entry: 10 };
    let expr = Hir;

    let result = instance.c_repeat_one_or_more(&expr, false).unwrap();
    assert_eq!(result.hole, 10);
    assert_eq!(result.entry, 10);
}

