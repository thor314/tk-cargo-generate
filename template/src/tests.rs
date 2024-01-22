mod demo {
  use arbitrary::Arbitrary;
  use log::{debug, info};
  use rstest::{fixture, rstest};
  // rstest provides features to take common context into tests, and set up small cases testing
  #[derive(Clone, Debug, Eq, PartialEq, Arbitrary)]
  struct Wb {
    b:     bool,
    count: usize,
  }
  // context setup function to be implicitly called by `workbench`
  #[fixture]
  fn count() -> usize { return 0usize; }
  // context setup function to be implicitly called by `test_workbench`
  #[fixture]
  fn wb(#[default(false)] b: bool, count: usize) -> Wb {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
    Wb { b, count }
  }

  // small-cases fuzzing
  // argument workbench will inherit the above function if names match; will generate 3x3 case-tests
  #[rstest] 
  #[case(0, true, true)]
  #[case(1, true, false)]
  fn test_workbench(
    wb: Wb,
    #[case] n: usize,
    #[case] b: bool,
    #[case] expected: bool,
  ) {
    info!("test logged, workbench is {workbench:?}");
    let wb_ = Wb { count: n, b };
    assert_eq!(wb == wb_, expected); // this will fail for case_1 cases
  }

  // ex 2 - baby fuzz; will generate 2x2 test cases
  #[rstest]
  fn test_enumerative(#[values(0, 4)] n: usize, #[values(7, 8)] m: usize) {
    assert!(n < m);
  }

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
