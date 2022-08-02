//! unit tests, see $root/tests/tests.rs for examples
use arbitrary::Arbitrary;
use quickcheck_macros::quickcheck;
use rstest::{fixture, rstest};
use test_log::test as ltest; // logs in tests, can wrap other test macros

#[test]
fn it_works() {
  let a = 2usize;
  assert_eq!(2 + 2, 4);
}