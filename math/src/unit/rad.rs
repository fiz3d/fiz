use num::traits::{Num, NumCast};
use std::f64;

use super::deg::{Deg, ToDeg};

/// ToRad is the canonical trait to use for taking input in radians.
///
/// For example the degrees type (Deg) implements the ToRad trait and thus
/// degrees can be given as a parameter to any input that seeks radians.
pub trait ToRad{
    type Output;

    /// to_rad returns these units in radians, performing conversion if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{Deg, ToRad};
    /// use fiz_math::Num;
    /// use std::fmt::Debug;
    ///
    /// fn walk<T: ToRad<Output=U>, U: Num+Debug>(x: T) {
    ///     println!("{:?}", x.to_rad().0)
    /// }
    /// walk(Deg(2.0));
    /// walk(Deg::<i16>(2));
    /// ```
    fn to_rad(self) -> Rad<Self::Output>;
}

/// Rad represents radians (the standard unit of angular measure).
///
/// # Examples
///
/// ```
/// use fiz_math::unit::Rad;
///
/// let x = Rad(1.0);
/// println!("{:?}", x);
/// ```
unit!(Rad);

impl<T: Num + NumCast> ToRad for Rad<T> {
    type Output = T;

    /// to_rad simply returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{Rad, ToRad};
    ///
    /// assert_eq!(Rad(1.0).to_rad(), Rad(1.0));
    /// ```
    fn to_rad(self) -> Rad<T> {
        self
    }
}

static FRAC_180_PI: f64 = 180.0 / f64::consts::PI;

impl<T: Num + NumCast> ToDeg for Rad<T> {
    type Output = T;

    /// to_deg converts these radians into degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use fiz_math::unit::{Rad, Deg, ToDeg};
    ///
    /// assert_eq!(Rad(2.0*PI).to_deg(), Deg(360.0));
    /// ```
    fn to_deg(self) -> Deg<T> {
        Deg(self.0 * (T::from(FRAC_180_PI).unwrap()))
    }
}
