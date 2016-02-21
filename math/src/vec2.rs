#![allow(dead_code)]

use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::{PartialEq, PartialOrd, Ordering};
pub use num::{Zero, One, Num};
use num;
use super::float::Float;
use super::Vec3;
use super::unit::{ToRad, Rad};
use std::fmt;
use clamp::Clamp;
use std::iter::IntoIterator;

/// Vec2 is a generic two-component vector type.
///
/// # Examples
///
/// ```
/// let x = fiz_math::Vec2(4.0f32, 8.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = fiz_math::Vec2(1u8, 5u8);
/// println!("{:?}", x);
/// ```
///
/// ```
/// use fiz_math::Vec2;
/// use fiz_math::unit::MM;
///
/// let x = Vec2(MM(1.0), MM(5.0));
/// let y = Vec2(MM(1.0), MM(5.1));
/// assert!(x.almost_equal(y, 0.1));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T>(pub T, pub T);

impl<T: Copy> IntoIterator for Vec2<T> {
    type Item = T;
    type IntoIter = Vec2Iterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        Vec2Iterator {
            v: self,
            index: 0,
        }
    }
}

pub struct Vec2Iterator<T> {
    v: Vec2<T>,
    index: usize,
}

impl<T: Copy> Iterator for Vec2Iterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let result = match self.index {
            0 => Some(self.v.0),
            1 => Some(self.v.1),
            _ => return None,
        };
        self.index += 1;
        result
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    /// fmt formats the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = fiz_math::Vec2(1u8, 5u8);
    /// assert_eq!(format!("{}", x), "Vec2(1, 5)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({}, {})", self.0, self.1)
    }
}

impl<T: One> One for Vec2<T> {
    /// one returns the one value for a vector whose component type implements the
    /// num::One trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::One;
    ///
    /// let x = fiz_math::Vec2::<f32>::one();
    /// ```
    ///
    /// ```
    /// use fiz_math::One;
    ///
    /// let x = fiz_math::Vec2::<i64>::one();
    /// ```
    fn one() -> Self {
        Vec2(T::one(), T::one())
    }
}

impl<T: Float> Vec2<T> {
    /// almost_equal tells if this vector is equal to the other given an absolute
    /// tolerence value (see the almost_equal function for more details).
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2::<f32>(1.0, 1.0);
    /// let b = Vec2::<f32>(0.9, 0.9);
    /// assert!(a.almost_equal(b, 0.1000001));
    /// assert!(!a.almost_equal(b, 0.1));
    /// ```
    pub fn almost_equal<N: num::Float>(self, other: Self, abs_tol: N) -> bool {
        self.0.almost_equal(other.0, abs_tol) && self.1.almost_equal(other.1, abs_tol)
    }

    /// is_nan tells if all of this vectors components are NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate fiz_math;
    /// use num::traits::Float;
    /// use fiz_math::Vec2;
    ///
    /// # fn main() {
    /// let n:f32 = Float::nan();
    /// assert!(Vec2(n, n).is_nan());
    /// assert!(!Vec2(n, 0.0).is_nan());
    /// # }
    /// ```
    pub fn is_nan(self) -> bool {
        self.0.is_nan() && self.1.is_nan()
    }

    /// round returns the nearest integer to a number. Round half-way cases away
    /// from 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// assert_eq!(Vec2(0.3, 1.3).round(), Vec2(0.0, 1.0))
    /// ```
    pub fn round(&self) -> Self {
        Vec2(self.0.round(), self.1.round())
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    /// add performs component-wise addition of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1, 2);
    /// let b = Vec2(4, 5);
    /// assert_eq!(a + b, Vec2(5, 7));
    /// ```
    fn add(self, _rhs: Self) -> Self {
        Vec2(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl<T: Add<Output = T> + Copy> Vec2<T> {
    /// add_scalar performs scalar addition on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1, 2);
    /// assert_eq!(a.add_scalar(1), Vec2(2, 3));
    /// ```
    pub fn add_scalar(self, _rhs: T) -> Self {
        Vec2(self.0 + _rhs, self.1 + _rhs)
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    /// neg returns the negated (i.e. inversed) vector self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// assert_eq!(-Vec2(1, 2), Vec2(-1, -2));
    /// ```
    fn neg(self) -> Self {
        Vec2(-self.0, -self.1)
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    /// sub performs component-wise subtraction of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1, 2);
    /// let b = Vec2(4, 5);
    /// assert_eq!(a - b, Vec2(-3, -3));
    /// ```
    fn sub(self, _rhs: Self) -> Self {
        Vec2(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl<T: Sub<Output = T> + Copy> Vec2<T> {
    /// sub_scalar performs scalar subtraction on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(2, 3);
    /// assert_eq!(a.sub_scalar(1), Vec2(1, 2));
    /// ```
    pub fn sub_scalar(self, _rhs: T) -> Self {
        Vec2(self.0 - _rhs, self.1 - _rhs)
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;

    /// mul performs component-wise multiplication of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1, 2);
    /// let b = Vec2(4, 5);
    /// assert_eq!(a * b, Vec2(4, 10));
    /// ```
    fn mul(self, _rhs: Self) -> Self {
        Vec2(self.0 * _rhs.0, self.1 * _rhs.1)
    }
}

impl<T: Mul<Output = T> + Copy> Vec2<T> {
    /// mul_scalar performs scalar multiplication on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(2, 3);
    /// assert_eq!(a.mul_scalar(2), Vec2(4, 6));
    /// ```
    pub fn mul_scalar(self, _rhs: T) -> Self {
        Vec2(self.0 * _rhs, self.1 * _rhs)
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Vec2<T>;

    /// div performs component-wise division of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(4, 5);
    /// let b = Vec2(1, 2);
    /// assert_eq!(a / b, Vec2(4, 2));
    /// ```
    fn div(self, _rhs: Self) -> Self {
        Vec2(self.0 / _rhs.0, self.1 / _rhs.1)
    }
}

impl<T: Div<Output = T> + Copy> Vec2<T> {
    /// div_scalar performs scalar division on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(2, 4);
    /// assert_eq!(a.div_scalar(2), Vec2(1, 2));
    /// ```
    pub fn div_scalar(self, _rhs: T) -> Self {
        Vec2(self.0 / _rhs, self.1 / _rhs)
    }
}

impl<T: Clamp<Elem = T> + Copy> Clamp for Vec2<T> {
    type Elem = T;

    /// clamp returns the vector with each element clamped to the range of
    /// [min, max].
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec2, Clamp};
    ///
    /// let a = Vec2(-2, 4);
    /// assert_eq!(a.clamp(-1, 2), Vec2(-1, 2));
    /// ```
    fn clamp(self, min: T, max: T) -> Self {
        Vec2(self.0.clamp(min, max), self.1.clamp(min, max))
    }
}

impl<T> AsRef<Vec2<T>> for Vec2<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: PartialOrd> Vec2<T> {
    /// any_less tells if any component of the other vector is less than any
    /// component of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(0, 1);
    /// assert!(a.any_less(Vec2(0, 2)));
    /// ```
    pub fn any_less<O: AsRef<Self>>(&self, other: O) -> bool {
        let o = other.as_ref();
        self.0 < o.0 || self.1 < o.1
    }

    /// any_greater tells if any component of the other vector is greater than
    /// any component of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(0, 2);
    /// assert!(a.any_greater(Vec2(0, 1)));
    /// ```
    pub fn any_greater<O: AsRef<Self>>(&self, other: O) -> bool {
        let o = other.as_ref();
        self.0 > o.0 || self.1 > o.1
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    /// eq tests for component-wise binary equality of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(4.0, 9.0);
    /// let b = Vec2(4.0, 9.00000000000000000000001);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(4, 5);
    /// let b = Vec2(4, 5);
    /// assert_eq!(a, b);
    /// ```
    fn eq(&self, _rhs: &Self) -> bool {
        self.0 == _rhs.0 && self.1 == _rhs.1
    }
}

impl<T: PartialOrd> PartialOrd for Vec2<T> {
    /// partial_cmp compares the two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1.0, 2.0);
    /// assert!(a < Vec2(1.1, 2.1));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 < other.0 && self.1 < other.1 {
            Some(Ordering::Less)
        } else if self.0 > other.0 && self.1 > other.1 {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> Vec2<T> {
    /// min returns a vector representing the smallest components of the `self`
    /// and `other` vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// assert_eq!(Vec2(0, 1).min(Vec2(-1, 0)), Vec2(-1, 0));
    /// ```
    pub fn min(self, other: Self) -> Self {
        Vec2(if self.0 < other.0 {
                 self.0
             } else {
                 other.0
             },
             if self.1 < other.1 {
                 self.1
             } else {
                 other.1
             })
    }

    /// max returns a vector representing the largest components of the `self`
    /// and `other` vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// assert_eq!(Vec2(0, 1).max(Vec2(-1, 0)), Vec2(0, 1));
    /// ```
    pub fn max(self, other: Self) -> Self {
        Vec2(if self.0 > other.0 {
                 self.0
             } else {
                 other.0
             },
             if self.1 > other.1 {
                 self.1
             } else {
                 other.1
             })
    }
}

impl<T: Zero> Zero for Vec2<T> {
    /// zero returns the zero-value for the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Zero, Vec2};
    ///
    /// let x = Vec2::<u8>::zero();
    /// let y = Vec2::<i64>::zero();
    /// let z = Vec2::<f32>::zero();
    /// let w = Vec2::<f64>::zero();
    /// ```
    fn zero() -> Self {
        Vec2(Zero::zero(), Zero::zero())
    }

    /// is_zero tests if the vector is equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Zero, Vec2};
    ///
    /// assert!(!Vec2(1i32, 0).is_zero());
    /// assert!(Vec2(0u8, 0).is_zero());
    /// assert!(!Vec2(1.0f32, 0.0).is_zero());
    /// assert!(Vec2(0.0f64, 0.0).is_zero());
    /// ```
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
}

impl<T: Num + Copy> Vec2<T> {
    /// dot returns the dot product of self and b. For length calculations use
    /// length or length_sq functions instead (for clarity).
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let x = Vec2(1, 2);
    /// assert_eq!(x.dot(x), 5);
    /// ```
    pub fn dot(self, b: Self) -> T {
        self.0 * b.0 + self.1 * b.1
    }

    /// length_sq returns the magnitude squared of this vector, useful primarily
    /// for comparing distances.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// assert_eq!(Vec2(1, 2).length_sq(), 5);
    /// ```
    pub fn length_sq(self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec2<T> {
    /// length returns the magnitude of this vector. Use length_sq instead when
    /// comparing distances, because it avoids the extra sqrt operation needed
    /// by this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec2, Float};
    ///
    /// let l = Vec2(1.0, 2.0).length();
    /// assert!(l.equal(2.23606797));
    /// ```
    pub fn length(self) -> T {
        self.length_sq().sqrt()
    }

    /// normalize returns the normalized (i.e. length/magnitude == 1) vector
    /// representing self. If the vector's length is zero and division by zero
    /// would occur, then None is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let n = Vec2(1.0, 10.0).normalize().unwrap();
    /// assert!(n.almost_equal(Vec2(0.1, 1.0), 1e-2));
    /// ```
    pub fn normalize(self) -> Option<Self> {
        let length = self.length();
        if length == T::zero() {
            None
        } else {
            Some(self.div_scalar(length))
        }
    }

    /// project returns a vector representing the projection of the `self` vector
    /// onto the `other` vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1.0, 2.0);
    /// let b = Vec2(1.0, 3.0);
    /// assert!(a.project(b).almost_equal(Vec2(0.7, 2.09), 0.01));
    /// ```
    pub fn project(self, other: Self) -> Self {
        other.mul_scalar(self.dot(other) / other.length_sq())
    }

    /// lerp returns a vector representing the linear interpolation between the
    /// `self` and `other` vectors. The parameter `t` is the amount to interpolate
    /// between the vectors (e.g. `0.0 - 1.0`).
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec2;
    ///
    /// let a = Vec2(1.25, 1.25);
    /// let b = Vec2(2.0, 2.0);
    /// assert_eq!(a.lerp(b, 0.25), Vec2(0.625, 0.625));
    /// ```
    pub fn lerp(self, other: Self, t: T) -> Self {
        self * other.mul_scalar(t)
    }
}

impl<F, T> Vec2<T>
    where F: Float,
          T: ToRad<Output = F>
{
    /// sphere_to_cart converts the given (inclination, azimuth) point on a sphere
    /// (`self`) of radius `r` to cartesian coordinates. It is implemented
    /// according to:
    ///
    /// http://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec2, Vec3};
    /// use fiz_math::unit::Rad;
    ///
    /// let r = Vec2(Rad(1.416), Rad(0.309)).sphere_to_cart(5.0);
    /// assert!(r.almost_equal(Vec3(Rad(4.706), Rad(1.502), Rad(0.770)), 0.001))
    /// ```
    pub fn sphere_to_cart(self, r: F) -> Vec3<Rad<F>> {
        let i = self.0.to_rad().0;
        let a = self.1.to_rad().0;
        Vec3(Rad(r * i.sin() * a.cos()),
             Rad(r * i.sin() * a.sin()),
             Rad(r * i.cos()))
    }
}

swizzle!(x, Vec2);
swizzle!(y, Vec2);
swizzle!(2, Vec2);
