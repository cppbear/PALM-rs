// Answer 0

#[test]
fn test_trans() {
    struct Translator {
        // Fields specific to the Translator can be defined here
    }

    struct HIR {
        trans: Translator,
    }

    impl HIR {
        fn new(trans: Translator) -> Self {
            HIR { trans }
        }

        fn trans(&self) -> &Translator {
            &self.trans
        }
    }

    let translator = Translator {};
    let hir = HIR::new(translator);
    
    let result = hir.trans();
    assert_eq!(std::ptr::addr_of!(*result), std::ptr::addr_of!(hir.trans));
}

