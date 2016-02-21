use num::traits::{Num, NumCast};
use std::f64;

use super::rad::{Rad, ToRad};

/// ToDeg is the canonical trait to use for taking input in degrees.
///
/// For example the radians type (Rad) implements the ToDeg trait and thus
/// radians can be given as a parameter to any input that seeks degrees.
pub trait ToDeg{
    type Output;

    /// to_deg returns these units in degrees, performing conversion if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{Rad, ToDeg};
    /// use fiz_math::Num;
    /// use std::fmt::Debug;
    ///
    /// fn walk<T: ToDeg<Output=U>, U: Num+Debug>(x: T) {
    ///     println!("{:?}", x.to_deg().0)
    /// }
    /// walk(Rad(2.0));
    /// walk(Rad::<i16>(2));
    /// ```
    fn to_deg(self) -> Deg<Self::Output>;
}

/// Deg represents degrees (a measurement of plane angle, representing 1/360th a
// full rotation).
///
/// # Examples
///
/// ```
/// use fiz_math::unit::Deg;
///
/// let x = Deg(1.0);
/// println!("{:?}", x);
/// ```
unit!(Deg);

impl<T: Num + NumCast> ToDeg for Deg<T> {
    type Output = T;

    /// to_deg simply returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{Deg, ToDeg};
    ///
    /// assert_eq!(Deg(1.0).to_deg(), Deg(1.0));
    /// ```
    fn to_deg(self) -> Deg<T> {
        self
    }
}

static FRAC_PI_180: f64 = f64::consts::PI / 180.0;

impl<T: Num + NumCast> ToRad for Deg<T> {
    type Output = T;

    /// to_rad converts these degrees into radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use fiz_math::unit::{Deg, Rad, ToRad};
    ///
    /// assert_eq!(Deg(360.0).to_rad(), Rad(2.0*PI));
    /// ```
    fn to_rad(self) -> Rad<T> {
        Rad(self.0 * (T::from(FRAC_PI_180).unwrap()))
    }
}
