#![cfg_attr(docsrs, doc(cfg(feature = "rayon")))]

use rayon::prelude::*;

use alloc::collections::LinkedList;

use crate::vec::Vec;

pub mod map;
pub mod set;

// This form of intermediate collection is also how Rayon collects `HashMap`.
// Note that the order will also be preserved!
fn collect<I: IntoParallelIterator>(iter: I) -> LinkedList<Vec<I::Item>> {
    iter.into_par_iter().collect_vec_list()
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::iter::Iterator;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut bool_0: bool = false;
    let mut u128_0: u128 = 5414u128;
    let mut bool_1: bool = false;
    let mut u128_1: u128 = 9499u128;
    let mut isize_0: isize = -1730isize;
    let mut isize_1: isize = 25477isize;
    let mut usize_0: usize = 6617usize;
    let mut isize_2: isize = -10046isize;
    let mut isize_2_ref_0: &isize = &mut isize_2;
    let mut isize_3: isize = -12923isize;
    let mut isize_4: isize = 8650isize;
    let mut isize_5: isize = 9389isize;
    let mut usize_1: usize = 1647usize;
    let mut usize_2: usize = 8873usize;
    let mut isize_6: isize = 5943isize;
    let mut isize_7: isize = 8589isize;
    let mut isize_8: isize = 10102isize;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &mut crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &crate::set::iter::Iter<isize> = &mut iter_0;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_1_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Iter::size_hint(iter_1_ref_0);
    let mut keys_1: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::clone(keys_0_ref_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::set::iter::Iter::size_hint(iter_0_ref_0);
    let mut usize_3: usize = crate::map::iter::Keys::count(keys_1);
    let mut option_0: std::option::Option<&isize> = crate::map::iter::Values::next(values_0_ref_0);
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    panic!("From RustyUnit with love");
}
}