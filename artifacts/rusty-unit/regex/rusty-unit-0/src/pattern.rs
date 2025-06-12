use std::str::pattern::{Pattern, SearchStep, Searcher};

use crate::re_unicode::{Matches, Regex};

#[derive(Debug)]
pub struct RegexSearcher<'r, 't> {
    haystack: &'t str,
    it: Matches<'r, 't>,
    last_step_end: usize,
    next_match: Option<(usize, usize)>,
}

impl<'r, 't> Pattern<'t> for &'r Regex {
    type Searcher = RegexSearcher<'r, 't>;

    fn into_searcher(self, haystack: &'t str) -> RegexSearcher<'r, 't> {
        RegexSearcher {
            haystack,
            it: self.find_iter(haystack),
            last_step_end: 0,
            next_match: None,
        }
    }
}

unsafe impl<'r, 't> Searcher<'t> for RegexSearcher<'r, 't> {
    #[inline]
    fn haystack(&self) -> &'t str {
        self.haystack
    }

    #[inline]
    fn next(&mut self) -> SearchStep {
        if let Some((s, e)) = self.next_match {
            self.next_match = None;
            self.last_step_end = e;
            return SearchStep::Match(s, e);
        }
        match self.it.next() {
            None => {
                if self.last_step_end < self.haystack().len() {
                    let last = self.last_step_end;
                    self.last_step_end = self.haystack().len();
                    SearchStep::Reject(last, self.haystack().len())
                } else {
                    SearchStep::Done
                }
            }
            Some(m) => {
                let (s, e) = (m.start(), m.end());
                if s == self.last_step_end {
                    self.last_step_end = e;
                    SearchStep::Match(s, e)
                } else {
                    self.next_match = Some((s, e));
                    let last = self.last_step_end;
                    self.last_step_end = s;
                    SearchStep::Reject(last, s)
                }
            }
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut bool_0: bool = true;
    let mut regexoptions_0: crate::re_builder::RegexOptions = crate::re_builder::RegexOptions::default();
    let mut execbuilder_0: crate::exec::ExecBuilder = crate::exec::ExecBuilder::new_options(regexoptions_0);
    let mut execbuilder_0_ref_0: &crate::exec::ExecBuilder = &mut execbuilder_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut u32_0: u32 = 5642u32;
    let mut usize_0: usize = 3748usize;
    let mut usize_1: usize = 9665usize;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut regexset_0: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::empty();
    let mut regexset_0_ref_0: &crate::re_set::bytes::RegexSet = &mut regexset_0;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut usize_2: usize = 9455usize;
    let mut usize_3: usize = 8006usize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut regexbuilder_0: crate::re_builder::unicode::RegexBuilder = crate::re_builder::unicode::RegexBuilder::new(str_0_ref_0);
    let mut regexbuilder_0_ref_0: &mut crate::re_builder::unicode::RegexBuilder = &mut regexbuilder_0;
    let mut bool_12: bool = false;
    let mut compiler_0: crate::compile::Compiler = crate::compile::Compiler::new();
    let mut compiler_1: crate::compile::Compiler = crate::compile::Compiler::reverse(compiler_0, bool_12);
    let mut compiler_1_ref_0: &mut crate::compile::Compiler = &mut compiler_1;
    let mut bool_13: bool = true;
    let mut str_1: &str = "LYTt1EwpsNo56sTmCJo";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut regexbuilder_1: crate::re_builder::unicode::RegexBuilder = crate::re_builder::unicode::RegexBuilder::new(str_1_ref_0);
    let mut regexbuilder_1_ref_0: &mut crate::re_builder::unicode::RegexBuilder = &mut regexbuilder_1;
    let mut suffixcacheentry_0: crate::compile::SuffixCacheEntry = crate::compile::SuffixCacheEntry::default();
    let mut suffixcacheentry_0_ref_0: &crate::compile::SuffixCacheEntry = &mut suffixcacheentry_0;
    let mut regexbuilder_2: &mut crate::re_builder::unicode::RegexBuilder = crate::re_builder::unicode::RegexBuilder::case_insensitive(regexbuilder_1_ref_0, bool_13);
    let mut regexbuilder_3: &mut crate::re_builder::unicode::RegexBuilder = crate::re_builder::unicode::RegexBuilder::size_limit(regexbuilder_0_ref_0, usize_3);
    let mut regexset_1: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::clone(regexset_0_ref_0);
    panic!("From RustyUnit with love");
}
}