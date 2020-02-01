//! This defines some traits so we can replace ops with method calls of the respective traits
//! (provided they're in scope) without worrying about argument types (hopefully)
//!
//! The traits are:
//! OverflowerAdd, OverflowerSub, OverflowerMul, OverflowerDiv, OverflowerRem,
//! OverflowerShl, OverflowerShr, OverflowerNeg, OverflowerAbs,
//! OverflowerAddAssign, OverflowerSubAssign, OverflowerMulAssign,
//! OverflowerDivAssign, etc.
//!
//! Also there is some trait / zero-sized-type dispatch machinery to implement
//! specialization on stable Rust.

#![feature(proc_macro_hygiene)]
#![deny(missing_docs, unsafe_code)]
#![no_std]

#[macro_use]
mod ops;
use core::iter::{Iterator, Product, Sum};
use core::ops::*;
pub use overflower_plugin::overflow;

// used internally to compute a signed maximum (or minimum) for saturating
trait SignedMax {
    fn signed_max(self) -> Self;
}

/// Overflow handling for summing iterators
pub trait OverflowerSum<A = Self> {
    /// wrapping sum
    fn sum_wrap<I: Iterator<Item = A>>(i: I) -> Self;

    /// panicking sum
    fn sum_panic<I: Iterator<Item = A>>(i: I) -> Self;

    /// saturating sum
    fn sum_saturate<I: Iterator<Item = A>>(i: I) -> Self;
}

/// Overflow handling for the product of all values in an iterator
pub trait OverflowerProduct<A = Self> {
    /// wrapping sum
    fn product_wrap<I: Iterator<Item = A>>(i: I) -> Self;

    /// panicking sum
    fn product_panic<I: Iterator<Item = A>>(i: I) -> Self;

    /// saturating sum
    fn product_saturate<I: Iterator<Item = A>>(i: I) -> Self;
}

op!(def2 OverflowerAdd, OverflowerAddAssign, Add, AddAssign,
    OverflowerAddTag, OverflowerAddAssignTag,
    OverflowerStdAddTag, OverflowerStdAddAssignTag,
    OverflowerAddKind, OverflowerAddAssignKind,
    OverflowerStdAddKind, OverflowerStdAddAssignKind,
    overflower_add_tag, overflower_add_assign_tag,
    add_wrap, add_panic, add_saturate,
    add_assign_wrap, add_assign_panic, add_assign_saturate,
    add, add_assign);
op!(def2 OverflowerSub, OverflowerSubAssign, Sub, SubAssign,
    OverflowerSubTag, OverflowerSubAssignTag,
    OverflowerStdSubTag, OverflowerStdSubAssignTag,
    OverflowerSubKind, OverflowerSubAssignKind,
    OverflowerStdSubKind, OverflowerStdSubAssignKind,
    overflower_sub_tag, overflower_sub_assign_tag,
    sub_wrap, sub_panic, sub_saturate,
    sub_assign_wrap, sub_assign_panic, sub_assign_saturate,
    sub, sub_assign);
op!(def2 OverflowerMul, OverflowerMulAssign, Mul, MulAssign,
    OverflowerMulTag, OverflowerMulAssignTag,
    OverflowerStdMulTag, OverflowerStdMulAssignTag,
    OverflowerMulKind, OverflowerMulAssignKind,
    OverflowerStdMulKind, OverflowerStdMulAssignKind,
    overflower_mul_tag, overflower_mul_assign_tag,
    mul_wrap, mul_panic, mul_saturate,
    mul_assign_wrap, mul_assign_panic, mul_assign_saturate,
    mul, mul_assign);
op!(def2 OverflowerDiv, OverflowerDivAssign, Div, DivAssign,
    OverflowerDivTag, OverflowerDivAssignTag,
    OverflowerStdDivTag, OverflowerStdDivAssignTag,
    OverflowerDivKind, OverflowerDivAssignKind,
    OverflowerStdDivKind, OverflowerStdDivAssignKind,
    overflower_div_tag, overflower_div_assign_tag,
    div_wrap, div_panic, div_saturate,
    div_assign_wrap, div_assign_panic, div_assign_saturate,
    div, div_assign);
op!(def2 OverflowerRem, OverflowerRemAssign, Rem, RemAssign,
    OverflowerRemTag, OverflowerRemAssignTag,
    OverflowerStdRemTag, OverflowerStdRemAssignTag,
    OverflowerRemKind, OverflowerRemAssignKind,
    OverflowerStdRemKind, OverflowerStdRemAssignKind,
    overflower_rem_tag, overflower_rem_assign_tag,
    rem_wrap, rem_panic, rem_saturate,
    rem_assign_wrap, rem_assign_panic, rem_assign_saturate,
    rem, rem_assign);
op!(def2 OverflowerShl, OverflowerShlAssign, Shl, ShlAssign,
    OverflowerShlTag, OverflowerShlAssignTag,
    OverflowerStdShlTag, OverflowerStdShlAssignTag,
    OverflowerShlKind, OverflowerShlAssignKind,
    OverflowerStdShlKind, OverflowerStdShlAssignKind,
    overflower_shl_tag, overflower_shl_assign_tag,
    shl_wrap, shl_panic, shl_saturate,
    shl_assign_wrap, shl_assign_panic, shl_assign_saturate,
    shl, shl_assign);
op!(def2 OverflowerShr, OverflowerShrAssign, Shr, ShrAssign,
    OverflowerShrTag, OverflowerShrAssignTag,
    OverflowerStdShrTag, OverflowerStdShrAssignTag,
    OverflowerShrKind, OverflowerShrAssignKind,
    OverflowerStdShrKind, OverflowerStdShrAssignKind,
    overflower_shr_tag, overflower_shr_assign_tag,
    shr_wrap, shr_panic, shr_saturate,
    shr_assign_wrap, shr_assign_panic, shr_assign_saturate,
    shr, shr_assign);
op!(def1 OverflowerNeg, Neg, OverflowerNegTag, OverflowerStdNegTag,
    OverflowerNegKind, OverflowerStdNegKind, overflower_neg_tag,
    neg_wrap, neg_panic, neg_saturate, neg);
op!(trait1 OverflowerAbs, abs, abs_wrap, abs_panic, abs_saturate);
op!(impls2 u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
op!(impls1 i8, i16, i32, i64, i128, isize);
op!(tagsp OverflowerSumTag, overflower_sum_tag, OverflowerSumKind);
op!(tagsp OverflowerProductTag, overflower_product_tag, OverflowerProductKind);
op!(tagsp OverflowerStdSumTag, overflower_sum_tag, OverflowerStdSumKind);
op!(tagsp OverflowerStdProductTag, overflower_product_tag, OverflowerStdProductKind);
impl<T> OverflowerStdSumKind for &T where T: Sum {}
impl<T> OverflowerSumKind for T where T: OverflowerSum {}
impl<T> OverflowerStdProductKind for &T where T: Product {}
impl<T> OverflowerProductKind for T where T: OverflowerProduct {}
op!(tagiterimpl OverflowerStdSumTag, Sum, sum_wrap, sum_panic, sum_saturate,
    sum, sum, sum);
op!(tagiterimpl OverflowerSumTag, OverflowerSum, sum_wrap, sum_panic, sum_saturate,
    sum_wrap, sum_panic, sum_saturate);
//op!(tagiterimpl OverflowerStdProductTag, Product, product_wrap, product_panic,
//    product_saturate, product, product, product);
//op!(tagiterimpl OverflowerProductTag, OverflowerProduct, product_wrap, product_panic,
//    product_saturate, product_wrap, product_panic, product_saturate);

/// This macro was used in the 0.9 version of overflower to forward `std` ops
/// implementations to the overflower traits, but with our new autoref-based
/// specialization machinery, this is no longer needed, so the macro is now a
/// deprecated no-op.
#[deprecated(
    since = "1.0.0",
    note = "This is no longer needed and will go away in some future version"
)]
#[macro_export]
macro_rules! impls {
    ($($tt:tt)*) => {};
}