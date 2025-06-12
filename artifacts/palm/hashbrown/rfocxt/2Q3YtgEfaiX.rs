use crate::{Equivalent, TryReserveError};
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign,
};
use core::{fmt, mem};
use map::make_hash;
use super::map::{self, HashMap, Keys};
use crate::raw::{Allocator, Global, RawExtractIf};
use crate::DefaultHashBuilder;
#[allow(dead_code)]
fn assert_covariance() {
    fn set<'new>(v: HashSet<&'static str>) -> HashSet<&'new str> {
        v
    }
    fn iter<'a, 'new>(v: Iter<'a, &'static str>) -> Iter<'a, &'new str> {
        v
    }
    fn into_iter<'new, A: Allocator>(
        v: IntoIter<&'static str, A>,
    ) -> IntoIter<&'new str, A> {
        v
    }
    fn difference<'a, 'new, A: Allocator>(
        v: Difference<'a, &'static str, DefaultHashBuilder, A>,
    ) -> Difference<'a, &'new str, DefaultHashBuilder, A> {
        v
    }
    fn symmetric_difference<'a, 'new, A: Allocator>(
        v: SymmetricDifference<'a, &'static str, DefaultHashBuilder, A>,
    ) -> SymmetricDifference<'a, &'new str, DefaultHashBuilder, A> {
        v
    }
    fn intersection<'a, 'new, A: Allocator>(
        v: Intersection<'a, &'static str, DefaultHashBuilder, A>,
    ) -> Intersection<'a, &'new str, DefaultHashBuilder, A> {
        v
    }
    fn union<'a, 'new, A: Allocator>(
        v: Union<'a, &'static str, DefaultHashBuilder, A>,
    ) -> Union<'a, &'new str, DefaultHashBuilder, A> {
        v
    }
    fn drain<'new, A: Allocator>(
        d: Drain<'static, &'static str, A>,
    ) -> Drain<'new, &'new str, A> {
        d
    }
}
