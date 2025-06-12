// Answer 0

#[test]
fn test_c_repeat_zero_or_more_greedy_ok() {
    use syntax::hir::{Hir, HirKind};
    use syntax::hir::ClassUnicodeRange;

    struct MockHir {
        kind: HirKind,
    }

    impl MockHir {
        fn new(kind: HirKind) -> Hir {
            Hir::new(kind)
        }
    }

    let mut compiler = Compiler::new();
    let expr = MockHir::new(HirKind::Class(hir::Class::Unicode(vec![ClassUnicodeRange::new('a', 'b')])));
    let greedy = true;

    if let Ok(patch) = compiler.c_repeat_zero_or_more(&expr, greedy) {
        assert_eq!(patch.entry, compiler.insts.len() - 1);
    } else {
        panic!("Expected Ok but got an error.");
    }
}

