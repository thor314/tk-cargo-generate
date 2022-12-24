use arbitrary::Arbitrary;
use quickcheck_macros::quickcheck;
use rstest::{fixture, rstest};
// logs in tests, can wrap other test macros
use test_log::test as ltest;

#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}
