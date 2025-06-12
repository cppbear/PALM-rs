// Answer 0

#[test]
fn test_c_repeat_zero_or_more_greedy_false() {
    struct TestStruct {
        insts: Vec<usize>,
    }

    impl TestStruct {
        fn push_split_hole(&mut self) -> usize {
            self.insts.push(0);
            self.insts.len() - 1
        }

        fn fill(&mut self, hole: usize, entry: usize) {
            self.insts[hole] = entry;
        }

        fn fill_split(&mut self, split: usize, entry_rep: Option<usize>, _: Option<usize>) -> usize {
            self.insts[split] = match entry_rep {
                Some(entry) => entry,
                None => 0,
            };
            self.insts.len() - 1
        }

        fn c(&self, _: &Hir) -> Result<Patch, ()> {
            Ok(Patch { hole: 0, entry: 0 })
        }

        fn c_repeat_zero_or_more(&mut self, expr: &Hir, greedy: bool) -> Result<Patch, ()> {
            let split_entry = self.insts.len();
            let split = self.push_split_hole();
            let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;

            self.fill(hole_rep, split_entry);
            let split_hole = if greedy {
                self.fill_split(split, Some(entry_rep), None)
            } else {
                self.fill_split(split, None, Some(entry_rep))
            };
            Ok(Patch { hole: split_hole, entry: split_entry })
        }
    }

    struct Hir;

    struct Patch {
        hole: usize,
        entry: usize,
    }

    let mut test_struct = TestStruct { insts: vec![] };
    let expr = Hir;

    let result = test_struct.c_repeat_zero_or_more(&expr, false);
    assert!(result.is_ok());

    let patch = result.unwrap();
    assert!(patch.hole < test_struct.insts.len());
    assert_eq!(patch.entry, test_struct.insts.len() - 1);
}

