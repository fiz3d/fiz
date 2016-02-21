extern crate num;

/// The default epsilon value used for floating point comparisons.
pub static EPSILON: f64 = 1.0E-8;

pub trait Float: num::Float{
    /// Tells if the two floating-point values `self` and `y` are considered equal
    /// within the specified `absolute == relative` tolerence value.
    ///
    /// The method of comparison used is described at:
    ///
    /// http://realtimecollisiondetection.net/blog/?p=89
    ///
    /// Also consider using the `equal` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Float;
    ///
    /// assert!(0.9.almost_equal(1.0, 0.1000001));
    /// assert!(0.9.almost_equal(1.0, 0.1));
    /// ```
    fn almost_equal<T: Float>(self, y: Self, abs_tol: T) -> bool;

    /// equal is short-hand for `self.almost_equal(y, fiz_math::EPSILON)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Float;
    ///
    /// assert!(1.00000001.equal(1.0));
    /// assert!(1.0.equal(1.0));
    /// assert!(!0.9.equal(1.0));
    /// ```
    fn equal(self, y: Self) -> bool;

    /// lerp performs a linear interpolation between `self` and `b`. The `t`
    /// parameter is a number in the range 0.0 - 1.0.
    ///
    /// The interpolation method is precise, as such it is guaranteed that
    /// `a.lerp(b, 1.0) == a`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Float;
    ///
    /// assert_eq!(0.0.lerp(10.0, 0.0), 0.0);
    /// assert_eq!(0.0.lerp(10.0, 0.5), 5.0);
    /// assert_eq!(0.0.lerp(10.0, 1.0), 10.0);
    /// ```
    fn lerp(self, b: Self, t: Self) -> Self;
}

impl<T: num::Float> Float for T {
    fn almost_equal<N: num::Float>(self, y: T, abs_tol: N) -> bool {
        let r = T::from(1.0).unwrap().max(self.abs().max(y.abs()));
        self == y || ((self - y).abs() <= T::from(abs_tol).unwrap() * r)
    }

    fn equal(self, y: T) -> bool {
        self.almost_equal(y, T::from(EPSILON).unwrap())
    }

    fn lerp(self, b: Self, t: Self) -> Self {
        (T::one() - t) * self + t * b
    }
}
