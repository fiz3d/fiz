extern crate num;

// Must re-export num for the unit! macro.
#[doc(hidden)]
pub use num as num_export;

#[macro_use]mod swizzle;
#[macro_use]pub mod unit_macro;

mod vec2;
mod vec3;
mod vec4;
mod float;
mod clamp;
pub mod unit;

pub use num::{Zero, One, Num};
pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::vec4::Vec4;
pub use self::float::{EPSILON, Float};
pub use self::clamp::Clamp;
