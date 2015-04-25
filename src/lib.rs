pub extern crate fiz_math as math;

mod test{
  #[allow(unused_imports)]
  use super::math;

  #[test]
  fn crate_names() {
    assert!(math::equal(1.0, 1.0));
  }
}
