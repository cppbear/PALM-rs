pub use self::imp::*;

#[cfg(feature = "perf-literal")]
mod imp;

#[allow(missing_docs)]
#[cfg(not(feature = "perf-literal"))]
mod imp {
    use regex_syntax::hir::literal::Literals;

    #[derive(Clone, Debug)]
    pub struct LiteralSearcher(());

    impl LiteralSearcher {
        pub fn empty() -> Self {
            LiteralSearcher(())
        }

        pub fn prefixes(_: Literals) -> Self {
            LiteralSearcher(())
        }

        pub fn suffixes(_: Literals) -> Self {
            LiteralSearcher(())
        }

        pub fn complete(&self) -> bool {
            false
        }

        pub fn find(&self, _: &[u8]) -> Option<(usize, usize)> {
            unreachable!()
        }

        pub fn find_start(&self, _: &[u8]) -> Option<(usize, usize)> {
            unreachable!()
        }

        pub fn find_end(&self, _: &[u8]) -> Option<(usize, usize)> {
            unreachable!()
        }

        pub fn is_empty(&self) -> bool {
            true
        }

        pub fn len(&self) -> usize {
            0
        }

        pub fn approximate_size(&self) -> usize {
            0
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_49() {
//     rusty_monitor::set_test_id(49);
//     let mut bool_0: bool = false;
//     let mut str_0: &str = "0HtIJ7";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut regexbuilder_0: crate::re_builder::bytes::RegexBuilder = crate::re_builder::bytes::RegexBuilder::new(str_0_ref_0);
//     let mut regexbuilder_0_ref_0: &mut crate::re_builder::bytes::RegexBuilder = &mut regexbuilder_0;
//     let mut bool_1: bool = true;
//     let mut bool_2: bool = false;
//     let mut compiler_0: crate::compile::Compiler = crate::compile::Compiler::new();
//     let mut compiler_1: crate::compile::Compiler = crate::compile::Compiler::bytes(compiler_0, bool_2);
//     let mut compiler_1_ref_0: &mut crate::compile::Compiler = &mut compiler_1;
//     let mut bool_3: bool = false;
//     let mut regexoptions_0: crate::re_builder::RegexOptions = crate::re_builder::RegexOptions::default();
//     let mut execbuilder_0: crate::exec::ExecBuilder = crate::exec::ExecBuilder::new_options(regexoptions_0);
//     let mut bool_4: bool = true;
//     let mut bool_5: bool = true;
//     let mut bool_6: bool = true;
//     let mut bool_7: bool = false;
//     let mut bool_8: bool = true;
//     let mut bool_9: bool = false;
//     let mut bool_10: bool = false;
//     let mut u32_0: u32 = 9974u32;
//     let mut usize_0: usize = 1282usize;
//     let mut usize_1: usize = 6090usize;
//     let mut bool_11: bool = true;
//     let mut bool_12: bool = false;
//     let mut emptyflags_0: crate::dfa::EmptyFlags = crate::dfa::EmptyFlags::default();
//     let mut emptyflags_0_ref_0: &crate::dfa::EmptyFlags = &mut emptyflags_0;
//     let mut execbuilder_1: crate::exec::ExecBuilder = crate::exec::ExecBuilder::bytes(execbuilder_0, bool_3);
//     let mut regexset_0: crate::re_set::unicode::RegexSet = crate::re_set::unicode::RegexSet::empty();
//     let mut regexbuilder_1: &mut crate::re_builder::bytes::RegexBuilder = crate::re_builder::bytes::RegexBuilder::multi_line(regexbuilder_0_ref_0, bool_0);
//     panic!("From RustyUnit with love");
// }
}