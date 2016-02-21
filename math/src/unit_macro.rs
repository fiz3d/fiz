/// unit creates a type representing a single unit.
///
/// # Examples
///
/// ```
/// #[macro_use(unit)]
/// extern crate fiz_math;
/// use std::fmt::Debug;
///
/// // GoldCoins are golden!
/// unit!(GoldCoins);
///
/// // SilverCoins are like gold coins, only worse!
/// unit!(SilverCoins);
///
/// fn collect<T: Debug>(coins: GoldCoins<T>) {
///     println!("We've collected {:?} golden coins!", coins.0);
/// }
///
/// fn main() {
///     let x = GoldCoins(5);
///     collect(x);
///
///     let y = SilverCoins(6);
///
///     // Try uncommenting the line below and see:
///     //
///     // error: mismatched types: expected `GoldCoins<_>`, found `SilverCoins<_>`"
///     //
///     //collect(y);
///
///     // add, subtract, multiply, divide:
///     assert_eq!(y + SilverCoins(4), SilverCoins(10));
///     assert_eq!(y - SilverCoins(3), SilverCoins(3));
///     assert_eq!(y * SilverCoins(3), SilverCoins(18));
///     assert_eq!(y / SilverCoins(2), SilverCoins(3));
///
///     // ordering and equality:
///     assert!(y < SilverCoins(7));
///     assert!(y > SilverCoins(5));
///     assert!(y <= SilverCoins(6));
///     assert!(y >= SilverCoins(6));
///     assert!(y == SilverCoins(6));
/// }
/// ```
///
/// ```
/// #[macro_use(unit)]
/// extern crate fiz_math;
/// extern crate num;
///
/// use num::traits::*;
/// use std::ops::*;
/// use std::cmp::*;
///
/// // check_impl defines a function (of the same name as the trait) whose
/// // single parameter is bound by the named trait, thus validating that a type
/// // does indeed implement a trait.
/// macro_rules! check_impl {
///     ($($name:ident),*) => ($(
///         #[allow(unused_variables)]
///         #[allow(non_snake_case)]
///         fn $name<T: $name>(x: T) {
///         }
///     )*);
/// }
///
/// check_impl!(Num, Float, PrimInt);
///
/// // Define our unit type.
/// unit!(MyUnit);
///
/// fn main() {
///     // All unit! implement the Num, Float, and PrimInt traits:
///     Num(MyUnit(0u8));
///     Num(MyUnit(0i32));
///     Num(MyUnit(0f32));
///
///     Float(MyUnit(0.0f32));
///     Float(MyUnit(0.0f64));
///
///     PrimInt(MyUnit(0u8));
///     PrimInt(MyUnit(0i32));
///     PrimInt(MyUnit(0isize));
/// }
/// ```
///
#[macro_export]
macro_rules! unit {
    ( $ident:ident ) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $ident<T>(pub T);

        unit!(impl_std_ops, $ident);
        unit!(impl_std_cmp, $ident);
        unit!(impl_num_traits, $ident);
        unit!(impl_num_traits_Float, $ident);
        unit!(impl_num_traits_PrimInt, $ident);
    };

    (impl_std_ops, $ident:ident ) => {
        impl<T: ::std::ops::Add<Output = T>> ::std::ops::Add for $ident<T> {
            type Output = Self;

            fn add(self, _rhs: Self) -> Self {
                $ident(self.0 + _rhs.0)
            }
        }

        impl<T: ::std::ops::Sub<Output = T>> ::std::ops::Sub for $ident<T> {
            type Output = Self;

            fn sub(self, _rhs: Self) -> Self {
                $ident(self.0 - _rhs.0)
            }
        }

        impl<T: ::std::ops::Mul<Output = T>> ::std::ops::Mul for $ident<T> {
            type Output = Self;

            fn mul(self, _rhs: Self) -> Self {
                $ident(self.0 * _rhs.0)
            }
        }

        impl<T: ::std::ops::Div<Output = T>> ::std::ops::Div for $ident<T> {
            type Output = Self;

            fn div(self, _rhs: Self) -> Self {
                $ident(self.0 / _rhs.0)
            }
        }

        impl<T: ::std::ops::Rem<Output = T>> ::std::ops::Rem for $ident<T> {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self { $ident(self.0 % rhs.0) }
        }

        impl<T: ::std::ops::Neg<Output = T>> ::std::ops::Neg for $ident<T> {
            type Output = Self;

            fn neg(self) -> Self::Output { $ident(-self.0) }
        }

        impl<T: ::std::ops::Not<Output = T>> ::std::ops::Not for $ident<T> {
            type Output = Self;

            fn not(self) -> Self::Output { $ident(!self.0) }
        }

        impl<T: ::std::ops::BitAnd<Output = T>> ::std::ops::BitAnd for $ident<T> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output { $ident(self.0 & rhs.0) }
        }

        impl<T: ::std::ops::BitOr<Output = T>> ::std::ops::BitOr for $ident<T> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output { $ident(self.0 | rhs.0) }
        }

        impl<T: ::std::ops::BitXor<Output = T>> ::std::ops::BitXor for $ident<T> {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output { $ident(self.0 ^ rhs.0) }
        }

        impl<T: ::std::ops::Shl<usize>> ::std::ops::Shl<usize> for $ident<T> {
            type Output = $ident<T::Output>;

            fn shl(self, rhs: usize) -> Self::Output { $ident(self.0 << rhs) }
        }

        impl<T: ::std::ops::Shr<usize>> ::std::ops::Shr<usize> for $ident<T> {
            type Output = $ident<T::Output>;

            fn shr(self, rhs: usize) -> Self::Output { $ident(self.0 >> rhs) }
        }
    };

    (impl_std_cmp, $ident:ident ) => {
        impl<T: ::std::cmp::PartialEq> ::std::cmp::PartialEq for $ident<T> {
            fn eq(&self, _rhs: &Self) -> bool {
                self.0 == _rhs.0
            }
        }

        impl<T: ::std::cmp::Eq> ::std::cmp::Eq for $ident<T> {}

        impl<T: ::std::cmp::PartialOrd> ::std::cmp::PartialOrd for $ident<T> {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                if self.0 < other.0 {
                    Some(::std::cmp::Ordering::Less)
                } else if self.0 > other.0 {
                    Some(::std::cmp::Ordering::Greater)
                } else if self == other {
                    Some(::std::cmp::Ordering::Equal)
                } else {
                    None
                }
            }
        }

        impl<T: ::std::cmp::Ord> ::std::cmp::Ord for $ident<T> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                if self.0 < other.0 {
                    ::std::cmp::Ordering::Less
                } else if self.0 > other.0 {
                    ::std::cmp::Ordering::Greater
                } else {
                    ::std::cmp::Ordering::Equal
                }
            }
        }
    };

    (impl_num_traits, $ident:ident ) => {
        impl<T: $crate::num_export::traits::Zero> $crate::num_export::traits::Zero for $ident<T> {
            fn zero() -> Self { $ident(T::zero()) }
            fn is_zero(&self) -> bool { self.0.is_zero() }
        }

        impl<T: $crate::num_export::traits::One> $crate::num_export::traits::One for $ident<T> {
            fn one() -> Self { $ident(T::one()) }
        }

        impl<T: $crate::num_export::traits::Num> $crate::num_export::traits::Num for $ident<T> {
            type FromStrRadixErr = T::FromStrRadixErr;
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                match T::from_str_radix(str, radix) {
                    Ok(x) => { Ok($ident(x)) }
                    Err(e) => { Err(e) }
                }
            }
        }

        impl<T: $crate::num_export::traits::ToPrimitive> $crate::num_export::traits::ToPrimitive for $ident<T> {
            fn to_i64(&self) -> Option<i64> { self.0.to_i64() }
            fn to_u64(&self) -> Option<u64> { self.0.to_u64() }
            fn to_isize(&self) -> Option<isize> { self.0.to_isize() }
            fn to_i8(&self) -> Option<i8> { self.0.to_i8() }
            fn to_i16(&self) -> Option<i16> { self.0.to_i16() }
            fn to_i32(&self) -> Option<i32> { self.0.to_i32() }
            fn to_usize(&self) -> Option<usize> { self.0.to_usize() }
            fn to_u8(&self) -> Option<u8> { self.0.to_u8() }
            fn to_u16(&self) -> Option<u16> { self.0.to_u16() }
            fn to_u32(&self) -> Option<u32> { self.0.to_u32() }
            fn to_f32(&self) -> Option<f32> { self.0.to_f32() }
            fn to_f64(&self) -> Option<f64> { self.0.to_f64() }
        }

        impl<T: $crate::num_export::traits::NumCast> $crate::num_export::traits::NumCast for $ident<T> {
            fn from<X: $crate::num_export::traits::ToPrimitive>(n: X) -> Option<Self> {
                match T::from(n) {
                    Some(x) => { Some($ident(x)) }
                    None => { None }
                }
            }
        }

        impl<T: $crate::num_export::traits::Bounded> $crate::num_export::traits::Bounded for $ident<T> {
            fn min_value() -> Self { $ident(T::min_value()) }
            fn max_value() -> Self { $ident(T::max_value()) }
        }

        impl<T: $crate::num_export::traits::CheckedAdd<Output = T>> $crate::num_export::traits::CheckedAdd for $ident<T> {
            fn checked_add(&self, _rhs: &Self) -> Option<Self> {
                match self.0.checked_add(&_rhs.0) {
                    Some(x) => { Some($ident(x)) }
                    None => { None }
                }
            }
        }

        impl<T: $crate::num_export::traits::CheckedSub<Output = T>> $crate::num_export::traits::CheckedSub for $ident<T> {
            fn checked_sub(&self, _rhs: &Self) -> Option<Self> {
                match self.0.checked_sub(&_rhs.0) {
                    Some(x) => { Some($ident(x)) }
                    None => { None }
                }
            }
        }

        impl<T: $crate::num_export::traits::CheckedMul<Output = T>> $crate::num_export::traits::CheckedMul for $ident<T> {
            fn checked_mul(&self, _rhs: &Self) -> Option<Self> {
                match self.0.checked_mul(&_rhs.0) {
                    Some(x) => { Some($ident(x)) }
                    None => { None }
                }
            }
        }

        impl<T: $crate::num_export::traits::CheckedDiv<Output = T>> $crate::num_export::traits::CheckedDiv for $ident<T> {
            fn checked_div(&self, _rhs: &Self) -> Option<Self> {
                match self.0.checked_div(&_rhs.0) {
                    Some(x) => { Some($ident(x)) }
                    None => { None }
                }
            }
        }
    };

    (impl_num_traits_Float, $ident:ident ) => {
        impl<T: $crate::num_export::traits::Float> $crate::num_export::traits::Float for $ident<T> {
            fn nan() -> Self { $ident(T::nan()) }
            fn infinity() -> Self { $ident(T::infinity()) }
            fn neg_infinity() -> Self { $ident(T::neg_infinity()) }
            fn neg_zero() -> Self { $ident(T::neg_zero()) }
            fn min_value() -> Self { $ident(T::min_value()) }
            fn min_positive_value() -> Self { $ident(T::min_positive_value()) }
            fn max_value() -> Self { $ident(T::max_value()) }
            fn is_nan(self) -> bool { self.0.is_nan() }
            fn is_infinite(self) -> bool { self.0.is_infinite() }
            fn is_finite(self) -> bool { self.0.is_finite() }
            fn is_normal(self) -> bool { self.0.is_normal() }
            fn classify(self) -> ::std::num::FpCategory { self.0.classify() }
            fn floor(self) -> Self { $ident(self.0.floor()) }
            fn ceil(self) -> Self { $ident(self.0.ceil()) }
            fn round(self) -> Self { $ident(self.0.round()) }
            fn trunc(self) -> Self { $ident(self.0.trunc()) }
            fn fract(self) -> Self { $ident(self.0.fract()) }
            fn abs(self) -> Self { $ident(self.0.abs()) }
            fn signum(self) -> Self { $ident(self.0.signum()) }
            fn is_sign_positive(self) -> bool { self.0.is_sign_positive() }
            fn is_sign_negative(self) -> bool { self.0.is_sign_negative() }
            fn mul_add(self, a: Self, b: Self) -> Self { $ident(self.0.mul_add(a.0, b.0)) }
            fn recip(self) -> Self { $ident(self.0.recip()) }
            fn powi(self, n: i32) -> Self { $ident(self.0.powi(n)) }
            fn powf(self, n: Self) -> Self { $ident(self.0.powf(n.0)) }
            fn sqrt(self) -> Self { $ident(self.0.sqrt()) }
            fn exp(self) -> Self { $ident(self.0.exp()) }
            fn exp2(self) -> Self { $ident(self.0.exp2()) }
            fn ln(self) -> Self { $ident(self.0.ln()) }
            fn log(self, base: Self) -> Self { $ident(self.0.log(base.0)) }
            fn log2(self) -> Self { $ident(self.0.log2()) }
            fn log10(self) -> Self { $ident(self.0.log10()) }
            fn max(self, other: Self) -> Self { $ident(self.0.max(other.0)) }
            fn min(self, other: Self) -> Self { $ident(self.0.min(other.0)) }
            fn abs_sub(self, other: Self) -> Self { $ident(self.0.abs_sub(other.0)) }
            fn cbrt(self) -> Self { $ident(self.0.cbrt()) }
            fn hypot(self, other: Self) -> Self { $ident(self.0.hypot(other.0)) }
            fn sin(self) -> Self { $ident(self.0.sin()) }
            fn cos(self) -> Self { $ident(self.0.cos()) }
            fn tan(self) -> Self { $ident(self.0.tan()) }
            fn asin(self) -> Self { $ident(self.0.asin()) }
            fn acos(self) -> Self { $ident(self.0.acos()) }
            fn atan(self) -> Self { $ident(self.0.atan()) }
            fn atan2(self, other: Self) -> Self { $ident(self.0.atan2(other.0)) }
            fn sin_cos(self) -> (Self, Self) { let (x, y) = self.0.sin_cos(); ($ident(x), $ident(y)) }
            fn exp_m1(self) -> Self { $ident(self.0.exp_m1()) }
            fn ln_1p(self) -> Self { $ident(self.0.ln_1p()) }
            fn sinh(self) -> Self { $ident(self.0.sinh()) }
            fn cosh(self) -> Self { $ident(self.0.cosh()) }
            fn tanh(self) -> Self { $ident(self.0.tanh()) }
            fn asinh(self) -> Self { $ident(self.0.asinh()) }
            fn acosh(self) -> Self { $ident(self.0.acosh()) }
            fn atanh(self) -> Self { $ident(self.0.atanh()) }
            fn integer_decode(self) -> (u64, i16, i8) { self.0.integer_decode() }
        }
    };

    (impl_num_traits_PrimInt, $ident:ident ) => {
        impl<T: $crate::num_export::traits::PrimInt> $crate::num_export::traits::PrimInt for $ident<T> {
            fn count_ones(self) -> u32 { self.0.count_ones() }
            fn count_zeros(self) -> u32 { self.0.count_zeros() }
            fn leading_zeros(self) -> u32 { self.0.leading_zeros() }
            fn trailing_zeros(self) -> u32 { self.0.trailing_zeros() }
            fn rotate_left(self, n: u32) -> Self { $ident(self.0.rotate_left(n)) }
            fn rotate_right(self, n: u32) -> Self { $ident(self.0.rotate_right(n)) }
            fn signed_shl(self, n: u32) -> Self { $ident(self.0.signed_shl(n)) }
            fn signed_shr(self, n: u32) -> Self { $ident(self.0.signed_shr(n)) }
            fn unsigned_shl(self, n: u32) -> Self { $ident(self.0.unsigned_shl(n)) }
            fn unsigned_shr(self, n: u32) -> Self { $ident(self.0.unsigned_shr(n)) }
            fn swap_bytes(self) -> Self { $ident(self.0.swap_bytes()) }
            fn from_be(x: Self) -> Self { $ident(T::from_be(x.0)) }
            fn from_le(x: Self) -> Self { $ident(T::from_le(x.0)) }
            fn to_be(self) -> Self { $ident(self.0.to_be()) }
            fn to_le(self) -> Self { $ident(self.0.to_le()) }
            fn pow(self, exp: u32) -> Self { $ident(self.0.pow(exp)) }
        }
    };
}
