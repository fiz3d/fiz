#![allow(dead_code)]

use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::{PartialEq, PartialOrd, Ordering};
pub use num::{Zero, One, Num};
use num;
use super::float::Float;
use super::Vec2;
use super::unit::ToRad;
use std::fmt;
use clamp::Clamp;
use std::iter::IntoIterator;

/// Vec3 is a generic three-component (3D) vector type.
///
/// # Examples
///
/// ```
/// let x = fiz_math::Vec3(4.0f32, 8.0f32, 2.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = fiz_math::Vec3(1u8, 5u8, 2u8);
/// println!("{:?}", x);
/// ```
///
/// ```
/// use fiz_math::Vec3;
/// use fiz_math::unit::MM;
///
/// let x = Vec3(MM(1.0), MM(5.0), MM(2.0));
/// let y = Vec3(MM(1.0), MM(5.1), MM(1.9));
/// assert!(x.almost_equal(y, 0.1));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Copy> IntoIterator for Vec3<T> {
    type Item = T;
    type IntoIter = Vec3Iterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        Vec3Iterator {
            v: self,
            index: 0,
        }
    }
}

pub struct Vec3Iterator<T> {
    v: Vec3<T>,
    index: usize,
}

impl<T: Copy> Iterator for Vec3Iterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let result = match self.index {
            0 => Some(self.v.0),
            1 => Some(self.v.1),
            2 => Some(self.v.2),
            _ => return None,
        };
        self.index += 1;
        result
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    /// fmt formats the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = fiz_math::Vec3(1u8, 5u8, 2u8);
    /// assert_eq!(format!("{}", x), "Vec3(1, 5, 2)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl<T: One> One for Vec3<T> {
    /// one returns the one value for a vector whose component type implements the
    /// num::One trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::One;
    ///
    /// let x = fiz_math::Vec3::<f32>::one();
    /// ```
    ///
    /// ```
    /// use fiz_math::One;
    ///
    /// let x = fiz_math::Vec3::<i64>::one();
    /// ```
    fn one() -> Self {
        Vec3(T::one(), T::one(), T::one())
    }
}

impl<T: Float> Vec3<T> {
    /// almost_equal tells if this vector is equal to the other given an absolute
    /// tolerence value (see the almost_equal function for more details).
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3::<f32>(1.0, 1.0, 1.0);
    /// let b = Vec3::<f32>(0.9, 0.9, 0.9);
    /// assert!(a.almost_equal(b, 0.1000001));
    /// assert!(!a.almost_equal(b, 0.1));
    /// ```
    pub fn almost_equal<N: num::Float>(self, other: Self, abs_tol: N) -> bool {
        self.0.almost_equal(other.0, abs_tol) && self.1.almost_equal(other.1, abs_tol) &&
        self.2.almost_equal(other.2, abs_tol)
    }

    /// is_nan tells if all of this vectors components are NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate fiz_math;
    /// use num::traits::Float;
    /// use fiz_math::Vec3;
    ///
    /// # fn main() {
    /// let n:f32 = Float::nan();
    /// assert!(Vec3(n, n, n).is_nan());
    /// assert!(!Vec3(n, 0.0, 0.0).is_nan());
    /// # }
    /// ```
    pub fn is_nan(self) -> bool {
        self.0.is_nan() && self.1.is_nan() && self.2.is_nan()
    }

    /// round returns the nearest integer to a number. Round half-way cases away
    /// from 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// assert_eq!(Vec3(0.3, 1.3, 2.0).round(), Vec3(0.0, 1.0, 2.0))
    /// ```
    pub fn round(&self) -> Self {
        Vec3(self.0.round(), self.1.round(), self.2.round())
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    /// add performs component-wise addition of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1, 2, 3);
    /// let b = Vec3(4, 5, 6);
    /// assert_eq!(a + b, Vec3(5, 7, 9));
    /// ```
    fn add(self, _rhs: Self) -> Self {
        Vec3(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2)
    }
}

impl<T: Add<Output = T> + Copy> Vec3<T> {
    /// add_scalar performs scalar addition on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1, 2, 3);
    /// assert_eq!(a.add_scalar(1), Vec3(2, 3, 4));
    /// ```
    pub fn add_scalar(self, _rhs: T) -> Self {
        Vec3(self.0 + _rhs, self.1 + _rhs, self.2 + _rhs)
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    /// neg returns the negated (i.e. inversed) vector self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// assert_eq!(-Vec3(1, 2, 3), Vec3(-1, -2, -3));
    /// ```
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    /// sub performs component-wise subtraction of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1, 2, 3);
    /// let b = Vec3(4, 5, 6);
    /// assert_eq!(a - b, Vec3(-3, -3, -3));
    /// ```
    fn sub(self, _rhs: Self) -> Self {
        Vec3(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.2)
    }
}

impl<T: Sub<Output = T> + Copy> Vec3<T> {
    /// sub_scalar performs scalar subtraction on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(2, 3, 4);
    /// assert_eq!(a.sub_scalar(1), Vec3(1, 2, 3));
    /// ```
    pub fn sub_scalar(self, _rhs: T) -> Self {
        Vec3(self.0 - _rhs, self.1 - _rhs, self.2 - _rhs)
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    /// mul performs component-wise multiplication of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1, 2, 3);
    /// let b = Vec3(4, 5, 6);
    /// assert_eq!(a * b, Vec3(4, 10, 18));
    /// ```
    fn mul(self, _rhs: Self) -> Self {
        Vec3(self.0 * _rhs.0, self.1 * _rhs.1, self.2 * _rhs.2)
    }
}

impl<T: Mul<Output = T> + Copy> Vec3<T> {
    /// mul_scalar performs scalar multiplication on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1, 2, 3);
    /// assert_eq!(a.mul_scalar(2), Vec3(2, 4, 6));
    /// ```
    pub fn mul_scalar(self, _rhs: T) -> Self {
        Vec3(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs)
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Vec3<T>;

    /// div performs component-wise division of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(4, 5, 9);
    /// let b = Vec3(1, 2, 3);
    /// assert_eq!(a / b, Vec3(4, 2, 3));
    /// ```
    fn div(self, _rhs: Self) -> Self {
        Vec3(self.0 / _rhs.0, self.1 / _rhs.1, self.2 / _rhs.2)
    }
}

impl<T: Div<Output = T> + Copy> Vec3<T> {
    /// div_scalar performs scalar division on a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(2, 4, 6);
    /// assert_eq!(a.div_scalar(2), Vec3(1, 2, 3));
    /// ```
    pub fn div_scalar(self, _rhs: T) -> Self {
        Vec3(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs)
    }
}

impl<T: Clamp<Elem = T> + Copy> Clamp for Vec3<T> {
    type Elem = T;

    /// clamp returns the vector with each element clamped to the range of
    /// [min, max].
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec3, Clamp};
    ///
    /// let a = Vec3(-2, 4, -6);
    /// assert_eq!(a.clamp(-1, 2), Vec3(-1, 2, -1));
    /// ```
    fn clamp(self, min: T, max: T) -> Self {
        Vec3(self.0.clamp(min, max),
             self.1.clamp(min, max),
             self.2.clamp(min, max))
    }
}

impl<T> AsRef<Vec3<T>> for Vec3<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: PartialOrd> Vec3<T> {
    /// any_less tells if any component of the other vector is less than any
    /// component of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(0, 0, 1);
    /// assert!(a.any_less(Vec3(0, 0, 2)));
    /// ```
    pub fn any_less<O: AsRef<Self>>(&self, other: O) -> bool {
        let o = other.as_ref();
        self.0 < o.0 || self.1 < o.1 || self.2 < o.2
    }

    /// any_greater tells if any component of the other vector is greater than
    /// any component of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(0, 0, 2);
    /// assert!(a.any_greater(Vec3(0, 0, 1)));
    /// ```
    pub fn any_greater<O: AsRef<Self>>(&self, other: O) -> bool {
        let o = other.as_ref();
        self.0 > o.0 || self.1 > o.1 || self.2 > o.2
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    /// eq tests for component-wise binary equality of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(4.0, 5.0, 9.0);
    /// let b = Vec3(4.0, 5.0, 9.00000000000000000000001);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(4, 5, 9);
    /// let b = Vec3(4, 5, 9);
    /// assert_eq!(a, b);
    /// ```
    fn eq(&self, _rhs: &Self) -> bool {
        self.0 == _rhs.0 && self.1 == _rhs.1 && self.2 == _rhs.2
    }
}

impl<T: PartialOrd> PartialOrd for Vec3<T> {
    /// partial_cmp compares the two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1.0, 2.0, 3.0);
    /// assert!(a < Vec3(1.1, 2.1, 3.1));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 < other.0 && self.1 < other.1 && self.2 < other.2 {
            Some(Ordering::Less)
        } else if self.0 > other.0 && self.1 > other.1 && self.2 > other.2 {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> Vec3<T> {
    /// min returns a vector representing the smallest components of the `self`
    /// and `other` vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// assert_eq!(Vec3(0, 1, 2).min(Vec3(-1, 0, 3)), Vec3(-1, 0, 2));
    /// ```
    pub fn min(self, other: Self) -> Self {
        Vec3(if self.0 < other.0 {
                 self.0
             } else {
                 other.0
             },
             if self.1 < other.1 {
                 self.1
             } else {
                 other.1
             },
             if self.2 < other.2 {
                 self.2
             } else {
                 other.2
             })
    }

    /// max returns a vector representing the largest components of the `self`
    /// and `other` vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// assert_eq!(Vec3(0, 1, 2).max(Vec3(-1, 0, 3)), Vec3(0, 1, 3));
    /// ```
    pub fn max(self, other: Self) -> Self {
        Vec3(if self.0 > other.0 {
                 self.0
             } else {
                 other.0
             },
             if self.1 > other.1 {
                 self.1
             } else {
                 other.1
             },
             if self.2 > other.2 {
                 self.2
             } else {
                 other.2
             })
    }
}

impl<T: Zero> Zero for Vec3<T> {
    /// zero returns the zero-value for the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Zero, Vec3};
    ///
    /// let x = Vec3::<u8>::zero();
    /// let y = Vec3::<i64>::zero();
    /// let z = Vec3::<f32>::zero();
    /// let w = Vec3::<f64>::zero();
    /// ```
    fn zero() -> Self {
        Vec3(Zero::zero(), Zero::zero(), Zero::zero())
    }

    /// is_zero tests if the vector is equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Zero, Vec3};
    ///
    /// assert!(!Vec3(1i32, 0, 0).is_zero());
    /// assert!(Vec3(0u8, 0, 0).is_zero());
    /// assert!(!Vec3(1.0f32, 0.0, 0.0).is_zero());
    /// assert!(Vec3(0.0f64, 0.0, 0.0).is_zero());
    /// ```
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero() && self.2.is_zero()
    }
}

impl<T: Num + Copy> Vec3<T> {
    /// dot returns the dot product of self and b. For length calculations use
    /// length or length_sq functions instead (for clarity).
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// let x = Vec3(1, 2, 3);
    /// assert_eq!(x.dot(x), 14);
    /// ```
    pub fn dot(self, b: Self) -> T {
        self.0 * b.0 + self.1 * b.1 + self.2 * b.2
    }

    /// length_sq returns the magnitude squared of this vector, useful primarily
    /// for comparing distances.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Vec3;
    ///
    /// assert_eq!(Vec3(1, 2, 3).length_sq(), 14);
    /// ```
    pub fn length_sq(self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec3<T> {
    /// length returns the magnitude of this vector. Use length_sq instead when
    /// comparing distances, because it avoids the extra sqrt operation needed
    /// by this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec3, Float};
    ///
    /// let l = Vec3(1.0, 2.0, 3.0).length();
    /// assert!(l.equal(3.74165738));
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
    /// use fiz_math::Vec3;
    ///
    /// let n = Vec3(1.0, 10.0, 100.0).normalize().unwrap();
    /// assert!(n.almost_equal(Vec3(0.01, 0.1, 1.0), 1e-2));
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
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1.0, 2.0, 4.0);
    /// let b = Vec3(1.0, 2.0, 3.0);
    /// assert!(a.project(b).almost_equal(Vec3(1.21, 2.42, 3.64), 0.01));
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
    /// use fiz_math::Vec3;
    ///
    /// let a = Vec3(1.25, 1.25, 1.25);
    /// let b = Vec3(2.0, 2.0, 2.0);
    /// assert_eq!(a.lerp(b, 0.25), Vec3(0.625, 0.625, 0.625));
    /// ```
    pub fn lerp(self, other: Self, t: T) -> Self {
        self * other.mul_scalar(t)
    }
}

impl<F, T> Vec3<T>
    where F: Float,
          T: Float + ToRad<Output = F>
{
    /// cart_to_sphere converts the given cartesian coordinate space point `self`
    /// into spherical coordinates in the form of a radius, (inclination, azimuth)
    /// pair. It is implemented according to:
    ///
    /// http://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::{Vec2, Vec3, Float};
    /// use fiz_math::unit::Rad;
    ///
    /// let inputRadius = 5.0;
    /// let inputSphere = Vec2(Rad(1.416), Rad(0.309));
    ///
    /// // Spherical to Cartesian
    /// let cart = inputSphere.sphere_to_cart(inputRadius);
    ///
    /// // Back to Spherical
    /// let (radius, sphere) = cart.cart_to_sphere();
    ///
    /// assert!(radius.equal(inputRadius));
    /// assert!(sphere.almost_equal(inputSphere, 0.001));
    /// ```
    pub fn cart_to_sphere(self) -> (F, Vec2<T>) {
        let r = self.length();
        let theta = (self.2 / r).acos();
        let phi = self.1.atan2(self.0);
        (r.to_rad().0, Vec2(theta, phi))
    }
}

swizzle!(x, Vec3);
swizzle!(y, Vec3);
swizzle!(z, Vec3);
swizzle!(2, Vec3);
swizzle!(3, Vec3);
