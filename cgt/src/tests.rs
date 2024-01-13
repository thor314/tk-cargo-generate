mod demo {
  use arbitrary::Arbitrary;
  use log::info;
  use rstest::{fixture, rstest};
  use test_log::test as ltest; // enables logging in tests
  // rstest provides features to take common context into tests, and set up small cases testing
  /// Some context to take into test functions, via `rstest`
  #[derive(Clone, Debug, Eq, PartialEq, Arbitrary)]
  struct TestWorkbench {
    b:     bool,
    count: usize,
  }
  // context setup function to be implicitly called by `workbench`
  #[fixture]
  fn count() -> usize { 0 }
  // context setup function to be implicitly called by `test_workbench`
  #[fixture]
  fn workbench(#[default(false)] b: bool, count: usize) -> TestWorkbench {
    TestWorkbench { b, count }
  }

  // small-cases fuzzing
  #[rstest] // argument workbench will inherit the above function if names match; will generate 3x3 case-tests
  #[case(0, true, true)]
  #[case(1, true, false)]
  #[ltest] // must follow rstest
  fn test_workbench(
    workbench: TestWorkbench,
    #[case] n: usize,
    #[case] b: bool,
    #[case] expected: bool,
  ) {
    info!("test logged, workbench is {workbench:?}");
    let wb = TestWorkbench { count: n, b };
    assert_eq!(workbench == wb, expected); // this will fail for some cases
  }

  // ex 2 - baby fuzz; will generate 2x2 test cases
  #[rstest]
  fn test_enumerative(#[values(0, 4)] n: usize, #[values(7, 8)] m: usize) { assert!(n < m) }

  // fuzz test
  fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs.iter() {
      rev.insert(0, x.clone())
    }
    rev
  }

  // fuzz, declare quickcheck on any argument implementing Arbitrary
  #[quickcheck_macros::quickcheck]
  fn prop(xs: Vec<u32>) -> bool { xs == reverse(&reverse(&xs)) }
}
