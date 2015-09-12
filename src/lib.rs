pub extern crate fiz_math as math;

mod test{
  #[allow(unused_imports)]
  use super::math::Float;

  #[test]
  fn crate_names() {
    assert!(1.0.equal(1.0));
  }
}
